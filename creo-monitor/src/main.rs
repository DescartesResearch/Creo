use std::path::Path;
use std::sync::Arc;

use creo_monitor::api::APIServer;
use creo_monitor::cgroup::{self, ContainerScanner};
use creo_monitor::container::ContainerDMetaDataProvider;
use creo_monitor::containerd::services::events::v1::SubscribeRequest;
use creo_monitor::containerd::services::events::v1::events_client::EventsClient;
use creo_monitor::error::{Error, Result};
use creo_monitor::persistence::Persister;
use creo_monitor::stats::CollectedStats;

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
        println!("Received event: {:?}", message);
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
