use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use prost::Message;
use prost_types::Any;
use tonic::metadata::MetadataValue;
use tonic::transport::Channel;

use crate::cgroup::{self, MonitoredContainer};
use crate::container::ContainerID;
use crate::containerd::events::{ContainerUpdate, TaskDelete, TaskStart};
use crate::containerd::services::containers::v1::GetContainerRequest;
use crate::containerd::services::containers::v1::containers_client::ContainersClient;
use crate::containerd::services::events::v1::SubscribeRequest;
use crate::containerd::services::events::v1::events_client::EventsClient;
use crate::containerd::services::namespaces::v1::ListNamespacesRequest;
use crate::containerd::services::namespaces::v1::namespaces_client::NamespacesClient;
use crate::containerd::services::tasks::v1::tasks_client::TasksClient;
use crate::containerd::v1::types::Status;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to connect to socket `{path}`: {source}")]
    SocketConnect {
        path: PathBuf,
        #[source]
        source: tonic::transport::Error,
    },
    #[error("failed to subscribe to events service: {0}")]
    Subscribe(#[source] Box<tonic::Status>),
    #[error("failed to receive event message: {0}")]
    EventMessage(#[source] Box<tonic::Status>),
    #[error("unknown event type `{type_url}` (value={value:?})")]
    UnknownEvent { type_url: String, value: Vec<u8> },
    #[error("failed to decode event type `{type_url}`: {source}")]
    EventDecode {
        type_url: String,
        #[source]
        source: prost::DecodeError,
    },
}

pub struct Discoverer {
    socket_path: PathBuf,
    join_handles: Vec<tokio::task::JoinHandle<Result<(), Error>>>,
}

impl Discoverer {
    pub fn new(socket_path: PathBuf) -> Self {
        Self {
            socket_path,
            join_handles: Vec::default(),
        }
    }

    pub async fn start(
        &mut self,
        monitor: Arc<cgroup::Monitor>,
        rootfs: PathBuf,
        cgroup_root: PathBuf,
        metadata_tx: tokio::sync::mpsc::Sender<(ContainerID, HashMap<String, String>)>,
    ) -> Result<(), Error> {
        let (container_tx, rx) = tokio::sync::mpsc::channel::<ContainerTask>(10);
        self.join_handles.push(tokio::spawn(add_container_task(
            rx,
            rootfs,
            cgroup_root,
            Arc::clone(&monitor),
        )));
        self.join_handles.push({
            let channel = crate::grpc::channel_for_unix_socket(&self.socket_path)
                .await
                .map_err(|source| Error::SocketConnect {
                    path: self.socket_path.clone(),
                    source,
                })?;
            let event_client = EventsClient::new(channel.clone());
            let container_client = ContainersClient::new(channel);
            let container_tx = container_tx.clone();
            let metadata_tx = metadata_tx.clone();
            tokio::spawn(events_task(
                event_client,
                container_client,
                Arc::clone(&monitor),
                container_tx,
                metadata_tx,
            ))
        });
        self.join_handles.push({
            let channel = crate::grpc::channel_for_unix_socket(&self.socket_path)
                .await
                .map_err(|source| Error::SocketConnect {
                    path: self.socket_path.clone(),
                    source,
                })?;
            let namespace_client = NamespacesClient::new(channel.clone());
            let tasks_client = TasksClient::new(channel.clone());
            let containers_client = ContainersClient::new(channel);

            tokio::spawn(existing_containers_task(
                namespace_client,
                tasks_client,
                containers_client,
                container_tx,
                metadata_tx,
            ))
        });

        Ok(())
    }

    pub async fn join_all(&mut self) -> Result<(), Error> {
        for handle in self.join_handles.drain(..) {
            handle.await.expect("Tasked panicked")?;
        }

        Ok(())
    }
}

async fn add_container_task(
    mut rx: tokio::sync::mpsc::Receiver<ContainerTask>,
    rootfs: PathBuf,
    cgroup_root: PathBuf,
    monitor: Arc<cgroup::Monitor>,
) -> Result<(), Error> {
    let mut line = String::with_capacity(255);
    while let Some(container_task) = rx.recv().await {
        line.clear();
        let path = rootfs.join(format!("proc/{}/cgroup", container_task.pid));
        match std::fs::File::open(&path) {
            Ok(f) => {
                let mut buf = BufReader::new(f);
                if let Ok(n) = buf.read_line(&mut line) {
                    if n == 0 {
                        log::warn!("empty cgroup file `{}`", path.display());
                        continue;
                    }
                    match parse_cgroup_line(line.as_str()) {
                        Ok(cgl) => {
                            if cgl.hierarchy_id != 0 {
                                log::warn!("expected hierarchy id 0, but was {}", cgl.hierarchy_id);
                                continue;
                            }

                            if !cgl.controller_list.is_empty() {
                                log::warn!(
                                    "expected empty controller list, but was {:?}",
                                    cgl.controller_list
                                );
                                continue;
                            }
                            let mut builder = cgroup::CollectorBuilder::default();
                            let cgroup_path =
                                cgl.cgroup_path.strip_prefix("/").unwrap_or(cgl.cgroup_path);
                            log::trace!("cgroup_path={}", cgroup_path);
                            let cgroup_prefix = cgroup_root.join(cgroup_path);
                            log::trace!("cgroup_prefix={}", cgroup_prefix.display());

                            builder.set_cpu_stat_file(cgroup_prefix.join("cpu.stat"));
                            builder.set_cpu_limit_file(cgroup_prefix.join("cpu.max"));
                            builder.set_memory_stat_file(cgroup_prefix.join("memory.stat"));
                            builder.set_memory_usage_file(cgroup_prefix.join("memory.current"));
                            builder.set_memory_limit_file(cgroup_prefix.join("memory.max"));
                            builder.set_io_stat_file(cgroup_prefix.join("io.stat"));
                            builder.set_network_stat_files(&[
                                rootfs.join(format!("proc/{}/net/dev", container_task.pid))
                            ]);

                            monitor.register_container(
                                container_task.id,
                                MonitoredContainer::new(
                                    container_task.id,
                                    vec![container_task.pid],
                                    builder.build(),
                                ),
                            );
                        }
                        Err(err) => {
                            log::error!("invalid cgroup file `{}`: {}", path.display(), err)
                        }
                    }
                }
            }
            Err(err) => {
                log::error!("Failed to open cgroup file `{}`: {}", path.display(), err);
            }
        }
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum CgroupLineError {
    #[error("invalid cgroup line format: {0}")]
    InvalidFormat(String),
    #[error("invalid hierarchy id in cgroup line: {0}")]
    InvalidHierarchyID(String),
    #[error("too many separators: {0}")]
    TooManySeparators(String),
}

pub struct CgroupLine<'a> {
    hierarchy_id: u32,
    controller_list: Vec<&'a str>,
    cgroup_path: &'a str,
}

fn parse_cgroup_line(line: &str) -> Result<CgroupLine<'_>, CgroupLineError> {
    let mut it = line.split(":");
    let hierarchy_id = it
        .next()
        .ok_or_else(|| CgroupLineError::InvalidFormat(line.to_owned()))?
        .parse::<u32>()
        .map_err(|_| CgroupLineError::InvalidHierarchyID(line.to_owned()))?;
    let controller_list = it
        .next()
        .ok_or_else(|| CgroupLineError::InvalidFormat(line.to_owned()))?;
    let controller_list: Vec<&str> = if controller_list.is_empty() {
        Vec::default()
    } else {
        controller_list.split(",").collect()
    };
    let cgroup_path = it
        .next()
        .ok_or_else(|| CgroupLineError::InvalidFormat(line.to_owned()))?;
    it.next().map_or(Ok(()), |_| {
        Err(CgroupLineError::TooManySeparators(line.to_owned()))
    })?;

    Ok(CgroupLine {
        hierarchy_id,
        controller_list,
        cgroup_path: cgroup_path.trim(),
    })
}

// Existing containers:
//  1. Namespaces Service:
//      ListNamespaces
//  2. Tasks Service per Namespace:
//      ListTasks (filter: status==running):
//  3. Container Service:
//      GetContainer: get labels
async fn existing_containers_task(
    mut namespace_client: NamespacesClient<Channel>,
    mut task_client: TasksClient<Channel>,
    mut container_client: ContainersClient<Channel>,
    container_tx: tokio::sync::mpsc::Sender<ContainerTask>,
    metadata_tx: tokio::sync::mpsc::Sender<(ContainerID, HashMap<String, String>)>,
) -> Result<(), Error> {
    match namespace_client
        .list(ListNamespacesRequest {
            filter: String::new(),
        })
        .await
    {
        Ok(response) => {
            let namespaces = response.into_inner();
            log::debug!("Found {} namespaces", namespaces.namespaces.len());
            for namespace in namespaces.namespaces {
                log::debug!(
                    "Requesting running tasks for namespace `{}`",
                    &namespace.name
                );
                let namespace_value = match MetadataValue::from_str(&namespace.name) {
                    Ok(val) => val,
                    Err(err) => {
                        log::error!(
                            "failed to create header value for namespace `{}`: {}",
                            namespace.name,
                            err
                        );
                        continue;
                    }
                };
                let mut request = tonic::Request::new(
                    crate::containerd::services::containers::v1::ListContainersRequest {
                        filters: Vec::default(),
                    },
                );
                request
                    .metadata_mut()
                    .insert("containerd-namespace", namespace_value.clone());
                let containers = match container_client.list(request).await {
                    Ok(response) => response.into_inner().containers,
                    Err(err) => {
                        log::error!(
                            "failed to list containers for namespace `{}`: {}",
                            &namespace.name,
                            err
                        );
                        continue;
                    }
                };
                log::debug!("Found {} existing containers", containers.len());
                let mut tasks = HashMap::with_capacity(containers.len());
                let mut metadata = Vec::with_capacity(containers.len());
                for container in containers {
                    let c_id = match ContainerID::from_str(&container.id) {
                        Ok(id) => id,
                        Err(err) => {
                            log::error!("failed to parse ContainerID: {}", err);
                            continue;
                        }
                    };
                    let mut request =
                        tonic::Request::new(crate::containerd::services::tasks::v1::GetRequest {
                            container_id: container.id,
                            exec_id: String::new(),
                        });
                    request
                        .metadata_mut()
                        .insert("containerd-namespace", namespace_value.clone());

                    let task = match task_client.get(request).await {
                        Ok(response) => match response.into_inner().process {
                            Some(task) => task,
                            None => {
                                log::warn!(
                                    "Received empty task for containerID `{}`",
                                    c_id.as_str()
                                );
                                continue;
                            }
                        },
                        Err(err) => {
                            log::warn!(
                                "failed to request task details for containerID `{}`: {}",
                                c_id.as_str(),
                                err
                            );
                            continue;
                        }
                    };
                    if task.status() != Status::Running {
                        continue;
                    }

                    tasks.insert(c_id, task.pid);
                    metadata.push((c_id, container.labels));
                }
                log::debug!("Found {} running containers", metadata.len());

                for container in metadata {
                    metadata_tx
                        .send(container)
                        .await
                        .expect("Reader side to still exist");
                }

                for task in tasks {
                    let task = ContainerTask {
                        id: task.0,
                        pid: task.1,
                    };
                    container_tx
                        .send(task)
                        .await
                        .expect("Reader side to still exist");
                }
            }
        }
        Err(err) => log::error!("failed to list containerd namespaces: {}", err),
    }

    Ok(())
}

pub struct ContainerTask {
    id: ContainerID,
    pid: u32,
}

async fn events_task(
    mut events_client: EventsClient<Channel>,
    mut container_client: ContainersClient<Channel>,
    monitor: Arc<cgroup::Monitor>,
    container_tx: tokio::sync::mpsc::Sender<ContainerTask>,
    metadata_tx: tokio::sync::mpsc::Sender<(ContainerID, HashMap<String, String>)>,
) -> Result<(), Error> {
    let mut stream = match events_client
        .subscribe(SubscribeRequest {
            filters: vec![
                r#"topic=="/tasks/start""#.to_owned(),
                r#"topic=="/tasks/delete""#.to_owned(),
                r#"topic=="/containers/update""#.to_owned(),
            ],
        })
        .await
        .map_err(|err| Error::Subscribe(Box::new(err)))
    {
        Ok(response) => response.into_inner(),
        Err(err) => {
            log::error!("{}", err);
            return Err(err);
        }
    };

    while let Some(msg) = stream
        .message()
        .await
        .map_err(|err| Error::EventMessage(Box::new(err)))?
    {
        log::debug!(
            "Received event: topic={}, namespace={}, timestamp={:?}",
            msg.topic,
            msg.namespace,
            msg.timestamp,
        );

        match msg.event {
            None => log::debug!("No event payload attached!"),
            Some(ref event) => match decode_event(event) {
                Ok(ev) => match ev {
                    Event::ContainerUpdate(container_update) => {
                        match ContainerID::from_str(&container_update.id) {
                            Ok(c_id) => {
                                log::debug!(
                                    "Received new labels for container `{}`: {:?}",
                                    &c_id,
                                    &container_update.labels
                                );
                                metadata_tx
                                    .send((c_id, container_update.labels))
                                    .await
                                    .expect("Reader side to still exist");
                            }
                            Err(err) => {
                                log::warn!(
                                    "failed to decode container ID from container update event: {}",
                                    err
                                )
                            }
                        }
                    }
                    Event::TaskStart(task_start) => {
                        match ContainerID::from_str(task_start.container_id.as_str()) {
                            Ok(id) => {
                                log::debug!(
                                    "Found new container with id `{}` and pid `{}`",
                                    &id,
                                    &task_start.pid
                                );

                                let mut request = tonic::Request::new(GetContainerRequest {
                                    id: task_start.container_id,
                                });
                                request.metadata_mut().insert(
                                    "containerd-namespace",
                                    MetadataValue::from_str(&msg.namespace)
                                        .expect("valid namespace"),
                                );

                                match container_client.get(request).await {
                                    Ok(response) => {
                                        if let Some(container) = response.into_inner().container {
                                            metadata_tx
                                                .send((id, container.labels))
                                                .await
                                                .expect("Reader side to still exist");
                                        }
                                    }
                                    Err(err) => {
                                        log::error!(
                                            "failed to get container info for container id `{}`: {}",
                                            &id,
                                            err
                                        );
                                    }
                                }
                                container_tx
                                    .send(ContainerTask {
                                        id,
                                        pid: task_start.pid,
                                    })
                                    .await
                                    .expect("Reader side to still exist");
                            }
                            Err(err) => {
                                log::warn!(
                                    "failed to decode container ID from task start event: {}",
                                    err
                                )
                            }
                        }
                    }
                    Event::TaskDelete(task_delete) => {
                        log::debug!(
                            "Event::TaskDelete(container_id={}, exec_id={})",
                            &task_delete.container_id,
                            &task_delete.id
                        );
                        // if exec_id == "" then the root exec_id of the task is deleted
                        // and as we only track the root tasks for each container, we have to stop
                        // tracking the container.
                        if task_delete.id.is_empty() {
                            match ContainerID::from_str(task_delete.container_id.as_str()) {
                                Ok(ref container_id) => {
                                    log::debug!(
                                        "Deleting container with container_id `{}` and pid `{}`",
                                        container_id,
                                        task_delete.pid
                                    );
                                    monitor.remove_container(container_id)
                                }
                                Err(err) => {
                                    log::warn!(
                                        "failed to decode container ID from task delete event: {}",
                                        err
                                    )
                                }
                            }
                        }
                    }
                },
                Err(err) => log::error!("{}", err),
            },
        }
    }

    Ok(())
}

pub enum Event {
    ContainerUpdate(ContainerUpdate),
    TaskStart(TaskStart),
    TaskDelete(TaskDelete),
}

fn decode_event(event: &Any) -> Result<Event, Error> {
    let ev = match event.type_url.as_str() {
        "containerd.events.ContainerUpdate" => {
            Event::ContainerUpdate(ContainerUpdate::decode(event.value.as_slice()).map_err(
                |source| Error::EventDecode {
                    type_url: event.type_url.clone(),
                    source,
                },
            )?)
        }
        "containerd.events.TaskStart" => {
            Event::TaskStart(TaskStart::decode(event.value.as_slice()).map_err(|source| {
                Error::EventDecode {
                    type_url: event.type_url.clone(),
                    source,
                }
            })?)
        }
        "containerd.events.TaskDelete" => Event::TaskDelete(
            TaskDelete::decode(event.value.as_slice()).map_err(|source| Error::EventDecode {
                type_url: event.type_url.clone(),
                source,
            })?,
        ),
        _ => {
            return Err(Error::UnknownEvent {
                type_url: event.type_url.clone(),
                value: event.value.clone(),
            });
        }
    };

    Ok(ev)
}
