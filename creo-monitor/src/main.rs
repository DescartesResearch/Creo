use std::path::Path;
use std::sync::Arc;

use creo_monitor::api::APIServer;
use creo_monitor::cgroup::{self, ContainerScanner};
use creo_monitor::container::ContainerDMetaDataProvider;
use creo_monitor::containerd::{
    events::{ContainerCreate, ContainerDelete, ContainerUpdate},
    events::{TaskCreate, TaskDelete, TaskExit, TaskStart},
    services::events::v1::{SubscribeRequest, events_client::EventsClient},
};
use creo_monitor::error::{Error, Result};
use creo_monitor::persistence::Persister;
use creo_monitor::stats::CollectedStats;
use prost::Message;
use prost_types::Any;

fn decode_event(event: &Any) {
    match event.type_url.as_str() {
        "containerd.events.ContainerCreate" => {
            match ContainerCreate::decode(event.value.as_slice()) {
                Ok(container_event) => println!("Container Create: {:?}", container_event),
                Err(err) => eprintln!("Failed to decode ContainerCreate: {err}"),
            }
        }
        "containerd.events.ContainerDelete" => {
            match ContainerDelete::decode(event.value.as_slice()) {
                Ok(container_event) => println!("Container Delete: {:?}", container_event),
                Err(err) => eprintln!("Failed to decode ContainerDelete: {err}"),
            }
        }
        "containerd.events.ContainerUpdate" => {
            match ContainerUpdate::decode(event.value.as_slice()) {
                Ok(container_event) => println!("Container Update: {:?}", container_event),
                Err(err) => eprintln!("Failed to decode ContainerUpdate: {err}"),
            }
        }
        "containerd.events.TaskCreate" => match TaskCreate::decode(event.value.as_slice()) {
            Ok(task_event) => println!("Task Create: {:?}", task_event),
            Err(err) => eprint!("Failed to decode TaskCreate: {err}"),
        },
        "containerd.events.TaskDelete" => match TaskDelete::decode(event.value.as_slice()) {
            Ok(task_event) => println!("Task Delete: {:?}", task_event),
            Err(err) => eprintln!("Failed to decode TaskDelete: {err}"),
        },
        "containerd.events.TaskStart" => match TaskStart::decode(event.value.as_slice()) {
            Ok(task_event) => println!("Task Start: {:?}", task_event),
            Err(err) => eprintln!("Failed to decode TaskStart: {err}"),
        },
        "containerd.events.TaskExit" => match TaskExit::decode(event.value.as_slice()) {
            Ok(task_event) => println!("Task Exit: {:?}", task_event),
            Err(err) => eprintln!("Failed to decode TaskExit: {err}"),
        },
        e => eprintln!("Unknown event type: {e}"),
    }
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let socket_path = Path::new("/var/run/containerd/containerd.sock");
    let channel = creo_monitor::grpc::channel_for_unix_socket(&socket_path).await?;
    let mut client = EventsClient::new(channel);

    let mut stream = client
        .subscribe(SubscribeRequest { filters: vec![] })
        .await?
        .into_inner();

    while let Some(message) = stream.message().await? {
        println!(
            "Received event: topic={}, namespace={}, timestamp={:?}",
            message.topic, message.namespace, message.timestamp
        );
        match message.event {
            Some(ref event) => decode_event(event),
            None => println!("No event attached"),
        };
    }

    Ok(())

    // let mut monitor = cgroup::Monitor::default();
    // let root_path: &std::path::Path = std::path::Path::new(creo_monitor::CGROUP_ROOT);
    //
    // let db_url =
    //     std::env::var("DATABASE_URL").expect("environment variable `DATABASE_URL` must be set");
    //
    // let db = Arc::new(
    //     creo_monitor::persistence::MySqlPersister::new(&db_url)
    //         .await
    //         .expect("failed to initialize persister"),
    // );
    // {
    //     let db = Arc::clone(&db);
    //     tokio::spawn(async move {
    //         let api = APIServer::new(db).await;
    //         api.listen("0.0.0.0:3000").await
    //     });
    // }
    // let (tx, mut rx) = tokio::sync::mpsc::channel::<Vec<CollectedStats>>(10);
    // {
    //     let db = Arc::clone(&db);
    //     tokio::spawn(async move {
    //         while let Some(stats) = rx.recv().await {
    //             if let Err(err) = db.persist_stats(&stats).await {
    //                 log::error!("{}", err);
    //             }
    //         }
    //     });
    // }
    //
    // let is_v2 = root_path.join("cgroup.controllers").exists();
    // let discoverer = if is_v2 {
    //     creo_monitor::cgroup::v2::Scanner {}
    // } else {
    //     panic!("cgroup v1 not supported yet!")
    // };
    //
    // let mut containerd_meta_provider = ContainerDMetaDataProvider::new().await;
    //
    // loop {
    //     let start = std::time::SystemTime::now();
    //     let timestamp = start
    //         .duration_since(std::time::UNIX_EPOCH)
    //         .expect("time to be later than UNIX EPOCH")
    //         .as_secs();
    //     log::info!("Finding containers@{timestamp}");
    //     discoverer
    //         .scan_path(root_path, &mut monitor, &mut containerd_meta_provider)
    //         .await
    //         .map_err(Error::DiscoverContainersError)?;
    //
    //     monitor.collect_stats(timestamp);
    //     let stats = monitor.drain_stats();
    //     tx.send(stats).await.unwrap();
    //
    //     let sleep = std::time::Duration::from_secs(1)
    //         - std::time::SystemTime::now()
    //             .duration_since(start)
    //             .expect("time to move forward");
    //
    //     log::debug!("Sleeping for {} ns", sleep.as_nanos());
    //     std::thread::sleep(sleep);
    // }
}
