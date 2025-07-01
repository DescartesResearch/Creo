mod dependencies;
use creo_monitor::container::{ContainerID, PodID};
use creo_monitor::persistence::{MySqlPersister, Persister};
use creo_monitor::stats::CpuStat;
use testcontainers::{ContainerAsync, GenericImage};

fn make_collected_stats(
    timestamp: u64,
    container_id: [u8; 64],
    pod_id: Option<[u8; 32]>,
    cpu_stat: Option<CpuStat>,
) -> creo_monitor::stats::CollectedStats {
    if let Some(pod) = pod_id {
        creo_monitor::stats::CollectedStats::Pod {
            timestamp,
            container_id: ContainerID::new(container_id).unwrap(),
            pod_id: PodID::new(pod).unwrap(),
            stats: creo_monitor::stats::ContainerStats::new(
                cpu_stat, None, None, None, None, None, None,
            ),
            container_metadata: None,
            pod_metadata: None,
        }
    } else {
        creo_monitor::stats::CollectedStats::Standalone {
            timestamp,
            container_id: ContainerID::new(container_id).unwrap(),
            stats: creo_monitor::stats::ContainerStats::new(
                cpu_stat, None, None, None, None, None, None,
            ),
            metadata: None,
        }
    }
}

async fn setup_persister()
-> Result<(MySqlPersister, ContainerAsync<GenericImage>), Box<dyn std::error::Error>> {
    let mysql_node = dependencies::start_mysql().await;
    let db_url = format!(
        "mysql://creo:creopassword@{}:{}/stats",
        mysql_node.get_host().await?,
        mysql_node.get_host_port_ipv4(3306).await?
    );
    Ok((MySqlPersister::new(&db_url).await?, mysql_node))
}

fn container_id_from_str(s: &str) -> [u8; 64] {
    let mut buf = [0u8; 64];
    buf[..s.len()].copy_from_slice(s.as_bytes());
    buf
}

fn make_simple_stat(timestamp: u64, id_str: &str) -> creo_monitor::stats::CollectedStats {
    make_collected_stats(timestamp, container_id_from_str(id_str), None, None)
}

#[tokio::test]
async fn test_persist_and_query_stats() -> Result<(), Box<dyn std::error::Error>> {
    let (persister, _mysql_node) = setup_persister().await?;
    let stats = [
        make_simple_stat(100, &format!("{:0<64}", 1)),
        make_simple_stat(200, &format!("{:0<64}", 2)),
    ];
    persister.persist_stats(&stats).await.unwrap();

    let queried = persister.query_stats_by_time_range(50, 300).await.unwrap();

    assert_eq!(queried.len(), 2);
    assert!(queried.iter().any(|s| s.container_id
        == "1000000000000000000000000000000000000000000000000000000000000000"
        && s.timestamp == 100));
    assert!(queried.iter().any(|s| s.container_id
        == "2000000000000000000000000000000000000000000000000000000000000000"
        && s.timestamp == 200));

    let empty = persister.query_stats_by_time_range(500, 600).await.unwrap();
    assert!(empty.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_persist_and_query_pod_stats() -> Result<(), Box<dyn std::error::Error>> {
    let (persister, _mysql_node) = setup_persister().await?;

    let pod_stat = make_collected_stats(
        150,
        *b"podcontainerpodcontainerpodcontainerpodcontainerpodcontainerpodc",
        Some(*b"podidpodidpodidpodidpodidpodidpo"),
        None,
    );

    persister.persist_stats(&[pod_stat]).await?;

    let queried = persister.query_stats_by_time_range(100, 200).await?;

    assert_eq!(queried.len(), 1);
    assert_eq!(queried[0].timestamp, 150);
    assert_eq!(
        queried[0].pod_id.as_deref(),
        Some("podidpodidpodidpodidpodidpodidpo")
    );
    assert_eq!(
        queried[0].container_id,
        "podcontainerpodcontainerpodcontainerpodcontainerpodcontainerpodc"
    );

    let empty = persister.query_stats_by_time_range(151, 200).await?;

    assert!(empty.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_persist_empty_stats() -> Result<(), Box<dyn std::error::Error>> {
    let (persister, _mysql_node) = setup_persister().await?;
    let result = persister.persist_stats(&[]).await;

    assert!(result.is_ok(), "Persisting empty stats should succeed");

    let queried = persister.query_stats_by_time_range(0, 1000).await?;
    assert!(queried.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_query_time_range_boundaries() -> Result<(), Box<dyn std::error::Error>> {
    let (persister, _mysql_node) = setup_persister().await?;

    let stat = make_collected_stats(
        123,
        *b"boundaryboundaryboundaryboundaryboundaryboundaryboundaryboundary",
        None,
        None,
    );

    persister.persist_stats(&[stat]).await?;

    // Boundaries should be inclusive
    let exact = persister.query_stats_by_time_range(123, 123).await?;
    assert_eq!(exact.len(), 1);

    Ok(())
}

#[tokio::test]
async fn test_parallel_stat_inserts() -> Result<(), Box<dyn std::error::Error>> {
    let (persister, _mysql_node) = setup_persister().await?;

    let prefix = "parallel";
    let tasks = (0..10).map(|i| {
        let persister = persister.clone();
        let container_id = format!("{}{:0width$}", prefix, i, width = 64 - prefix.len()); // pad to 64 chars
        let container_bytes = {
            let mut buf = [0u8; 64];
            buf[..container_id.len()].copy_from_slice(container_id.as_bytes());
            buf
        };

        tokio::spawn(async move {
            let stat = make_collected_stats(100 + i as u64 * 10, container_bytes, None, None);
            persister.persist_stats(&[stat]).await
        })
    });

    for task in tasks {
        let result = task.await;
        assert!(result.unwrap().is_ok());
    }

    let stats = persister.query_stats_by_time_range(0, 1000).await?;
    assert_eq!(stats.len(), 10);

    for i in 0..10 {
        let expected_id = format!("{}{:0width$}", prefix, i, width = 64 - prefix.len());
        assert!(stats.iter().any(|s| s.container_id == expected_id));
    }

    Ok(())
}
