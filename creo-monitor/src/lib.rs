use environment::RuntimeEnvironment;
use persistence::{MetadataPersister, StatsPersister};
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

/// Creo Monitor: A container monitoring tool that collects resource usage via cgroups
/// and persists data to a MySQL database.
///
/// This library provides the core functionality for discovering containers (e.g., via containerd),
/// monitoring their resource usage through cgroup files, and exposing metrics via an API.
pub mod api;
pub mod cgroup;
pub mod container;
pub mod discovery;
pub mod environment;
pub mod error;
pub mod fsutil;
pub mod grpc;
pub mod mountinfo;
pub mod persistence;

// in container it is really important to have "--privileged"
// check for container environment
//  check if /rootfs is there
//  check if namespaces are different
//  check if /proc/self/cgroup contains containerized parts
//  check if in container env, e.g., /.dockerenv
//
//  if in container and /rootfs missing: error
//  if not detected in container, but /proc/self/mountinfo returns multiple cgroup mounts -> warn
//  about missing "--privileged"
//  if in container and /rootfs there: everything as expected

// check /proc/<pid>/mountinfo for cgroup root

// Get PID of container
// check /proc/<pid>/cgroup for cgroup stat files
//  file format: <hierarchy-id>:<controller-list>:<cgroup-path>
//      <hierarchy-id>:
//          v1: arbitrary number
//          v2: always '0'
//      <controller-list>:
//          v1: comma-separated list of controllers, e.g., cpu,memory
//          v2: always empty, i.e., ''
//      <cgroup-path>:
//          v1: path of the controllers in the controller-list relative to the cgroup root
//          v2: unified path of all controllers relative to the cgroup root
//
// TODO: check if anything different from /rootfs/sys/fs/cgroup and /sys/fs/cgroup
// TODO: check if I can use /rootfs/var/run/containerd/containerd.sock
//
// Containerd API:
//  at startup: list namespaces -> for each namespace list tasks -> filter only running tasks ->
//  get pid from responses
//  subscribe to topic==/tasks/start -> read namespace from event -> read pid from event
//  subscribe to topic==/tasks/delete -> read namespace from event -> check if id (i.e. exec_id) is
//  "" (means that root exec_id is deleted) -> stop tracking

pub mod containerd {
    pub mod runc {
        pub mod v1 {
            tonic::include_proto!("containerd.runc.v1");
        }
    }
    pub mod v1 {
        pub mod types {
            tonic::include_proto!("containerd.v1.types");
        }
    }
    pub mod types {
        tonic::include_proto!("containerd.types");
    }
    pub mod events {
        tonic::include_proto!("containerd.events");
    }
    pub mod services {
        pub mod containers {
            pub mod v1 {
                tonic::include_proto!("containerd.services.containers.v1");
            }
        }
        pub mod events {
            pub mod v1 {
                tonic::include_proto!("containerd.services.events.v1");
            }
        }
        pub mod tasks {
            pub mod v1 {
                tonic::include_proto!("containerd.services.tasks.v1");
            }
        }
        pub mod namespaces {
            pub mod v1 {
                tonic::include_proto!("containerd.services.namespaces.v1");
            }
        }
    }
}

/// Runs the Creo Monitor application.
///
/// Initializes the container runtime discovery, cgroup monitoring, data persistence,
/// and API server.
///
/// # Returns
///
/// Returns `Ok(())` on successful execution, or an error if any component fails.
///
/// # Errors
///
/// Possible errors include:
/// - Missing environment variables (e.g., `DATABASE_URL`).
/// - Failure to connect to the database.
/// - Failure to initialize the container runtime discovery.
/// - I/O errors when reading system files (e.g., `/etc/machine-id`).
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let rootfs = std::env::var_os("ROOTFS_MOUNT_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/rootfs"));
    let runtime_env = environment::detect_runtime_environment(&rootfs);
    if matches!(runtime_env, RuntimeEnvironment::Container) && !rootfs.exists() {
        return Err(format!(
            "Detected container runtime environment, but missing host root mount at `{}`!",
            rootfs.display()
        )
        .into());
    }

    let rootfs = match runtime_env {
        RuntimeEnvironment::Container => rootfs,
        RuntimeEnvironment::Host => PathBuf::from("/"),
    };
    log::debug!("Final rootfs: {}", rootfs.display());
    let cgroup_root =
        mountinfo::detect_validated_cgroup2_mount_point(rootfs.join("proc/1/mountinfo"))?;
    let cgroup_root = rootfs.join(
        cgroup_root
            .strip_prefix("/")
            .expect("Mountinfo paths are absolute"),
    );
    log::debug!("Final Cgroup Root: {}", cgroup_root.display());

    let monitor = Arc::new(cgroup::Monitor::default());
    let mut discoverer = discovery::containerd::Discoverer::new(PathBuf::from(
        "/var/run/containerd/containerd.sock",
    ));

    let machine_id = container::MachineID::from_str(
        std::fs::read_to_string(rootfs.join("etc/machine-id"))?.trim(),
    )?;

    let hostname = std::fs::read_to_string(rootfs.join("etc/hostname"))
        .or_else(|_| std::fs::read_to_string("proc/sys/kernel/hostname"))?
        .trim()
        .to_owned();
    log::debug!("Hostname: {}", &hostname);
    let (metadata_tx, mut metadata_rx) =
        tokio::sync::mpsc::channel::<(container::ContainerID, HashMap<String, String>)>(15);

    let db_url =
        std::env::var("DATABASE_URL").expect("environment variable `DATABASE_URL` must be set");

    let db = sqlx::mysql::MySqlPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(10))
        .max_connections(10)
        .connect(&db_url)
        .await?;

    sqlx::migrate!().run(&db).await?;

    let metadata_persister =
        persistence::MySqlMetadataPersister::new(db.clone(), machine_id, hostname);
    tokio::spawn(async move {
        while let Some(metadata) = metadata_rx.recv().await {
            match metadata_persister.persist_metadata(metadata).await {
                Ok(_) => {}
                Err(err) => log::error!("failed to persist metadata: {}", err),
            }
        }
    });

    discoverer
        .start(Arc::clone(&monitor), rootfs, cgroup_root, metadata_tx)
        .await?;
    log::debug!("Started containerd discovery");

    let stats_persister = persistence::MySqlStatsPersister::new(db.clone(), machine_id);
    {
        let db = api::DB::new(db);
        tokio::spawn(async move {
            let api = api::APIServer::new(db).await;
            api.listen("0.0.0.0:3000").await
        });
    }
    let (tx, mut rx) = tokio::sync::mpsc::channel::<Vec<cgroup::stats::ContainerStatsEntry>>(10);
    {
        tokio::spawn(async move {
            while let Some(stats) = rx.recv().await {
                if let Err(err) = stats_persister.persist_stats(&stats).await {
                    log::error!("failed to persist stats: {}", err);
                }
            }
        });
    }

    let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
    loop {
        interval.tick().await;
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        log::trace!("Finding containers@{timestamp}");

        let monitor = Arc::clone(&monitor);

        let out = tokio::task::spawn_blocking(move || {
            let mut out = Vec::with_capacity(monitor.size());
            let before = std::time::Instant::now();
            monitor.collect_stats(timestamp, &mut out);
            let took = before.elapsed();
            log::trace!("collect_stats() took {} nanoseconds", took.as_nanos());
            out
        })
        .await
        .expect("spawn_blocking panicked");

        tx.send(out).await.expect("Reader side to still exist");
    }
}
