use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use creo_monitor::api::APIServer;
use creo_monitor::cgroup;
use creo_monitor::cgroup::stats::ContainerStatsEntry;
use creo_monitor::container::ContainerID;
use creo_monitor::persistence::{MetadataPersister, StatsPersister};
use sqlx::mysql::MySqlPoolOptions;

// TODO: check if anything different from /rootfs/sys/fs/cgroup and /sys/fs/cgroup
// TODO: check if I can use /rootfs/var/run/containerd/containerd.sock

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let rootfs = match env::var_os("ROOTFS_MOUNT_PATH") {
        Some(path) => PathBuf::from(&path),
        None => PathBuf::from("/rootfs"),
    };

    let runtime_env = creo_monitor::detect_runtime_environment(&rootfs);
    if matches!(runtime_env, creo_monitor::RuntimeEnvironment::Container) && !rootfs.exists() {
        // TODO: handle as error
        panic!("Detected container runtime environment, but missing host root mount!")
    }

    let rootfs = match runtime_env {
        creo_monitor::RuntimeEnvironment::Container => rootfs,
        creo_monitor::RuntimeEnvironment::Host => PathBuf::from("/"),
    };
    log::debug!("Final rootfs: {}", rootfs.display());
    let cgroup_root = creo_monitor::detect_cgroup_root(rootfs.join("proc/1/mountinfo"))?;
    log::debug!("Final Cgroup Root: {}", cgroup_root.display());

    let monitor = Arc::new(cgroup::Monitor::default());
    let mut discoverer = creo_monitor::discovery::containerd::Discoverer::new(PathBuf::from(
        "/var/run/containerd/containerd.sock",
    ));

    let machine_id = {
        let machine_id_str = std::fs::read_to_string(rootfs.join("etc/machine-id"))?;
        let machine_id_str = machine_id_str.trim();
        log::debug!("Read machine id from file: {}", &machine_id_str);
        creo_monitor::container::MachineID::from_str(machine_id_str)?
    };
    let hostname = {
        let hostname_str = std::fs::read_to_string(rootfs.join("etc/hostname"))?;
        hostname_str.trim().to_owned()
    };
    log::debug!("Hostname: {}", &hostname);
    let (metadata_tx, mut metadata_rx) =
        tokio::sync::mpsc::channel::<(ContainerID, HashMap<String, String>)>(15);

    let db_url =
        std::env::var("DATABASE_URL").expect("environment variable `DATABASE_URL` must be set");

    let db = MySqlPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(10))
        .max_connections(10)
        .connect(&db_url)
        .await?;

    sqlx::migrate!().run(&db).await?;

    let metadata_persister =
        creo_monitor::persistence::MySqlMetadataPersister::new(db.clone(), machine_id, hostname);
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

    let stats_persister =
        creo_monitor::persistence::MySqlStatsPersister::new(db.clone(), machine_id);
    {
        let db = creo_monitor::api::DB::new(db);
        tokio::spawn(async move {
            let api = APIServer::new(db).await;
            api.listen("0.0.0.0:3000").await
        });
    }
    let (tx, mut rx) = tokio::sync::mpsc::channel::<Vec<ContainerStatsEntry>>(10);
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
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time to be later than UNIX EPOCH")
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
