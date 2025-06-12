use chrono::{Duration, NaiveDateTime};
use local_encoding::Encoder;
use ndarray::Array1;
use std::collections::{BTreeMap, HashMap};
use tokio::io::AsyncBufReadExt;

use crate::stats::levene::levene_test;

use super::{path_to_str, BenchmarkConfig, Result};

pub async fn aggregate(
    benchmark_config: &BenchmarkConfig,
    lang_dir: impl AsRef<std::path::Path>,
) -> Result<()> {
    let lang_dir = lang_dir.as_ref();

    let mut lang_it = tokio::fs::read_dir(&lang_dir).await?;
    while let Some(handler_dir) = lang_it.next_entry().await? {
        let path = handler_dir.path();
        if !path.is_dir() {
            continue;
        }
        let benchmarks = path.join("benchmarks");
        if !benchmarks.is_dir() {
            log::warn!(
                "Missing benchmarks directory for path `{}`! Skipping...",
                path.display()
            );
            continue;
        }
        let mut load_level_it = tokio::fs::read_dir(benchmarks).await?;
        let mut load_level_results: HashMap<_, Vec<f64>> = HashMap::new();
        while let Some(load_level) = load_level_it.next_entry().await? {
            let result =
                test_load_level(&load_level.path(), benchmark_config.benchmark_duration).await?;
            for (key, value) in result {
                let entry = load_level_results.entry(key).or_default();
                entry.push(value);
            }
        }
        let utilization: BTreeMap<_, f64> =
            BTreeMap::from_iter(load_level_results.iter().map(|(key, values)| {
                let n = values.len();
                let avg = values.iter().sum::<f64>() / (n as f64);
                (*key, avg)
            }));
        let utilization_file = std::fs::File::create(path.join("utilization.yml"))?;
        serde_yaml::to_writer(&utilization_file, &utilization)?;
    }

    Ok(())
}

#[derive(serde::Deserialize, Debug)]
struct PrometheusAPIResponse {
    pub status: String,
    pub data: PrometheusAPIData,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PrometheusAPIData {
    result: Vec<PrometheusAPIDataResult>,
}

#[derive(serde::Deserialize, Debug)]
struct PrometheusAPIDataResult {
    values: Vec<(i64, String)>,
}

trait PrometheusMetric {
    fn to_string(&self) -> &'static str;
    fn as_key(&self) -> &'static str;
    fn is_required(&self) -> bool;
}

struct PrometheusCPU;
impl PrometheusMetric for PrometheusCPU {
    fn to_string(&self) -> &'static str {
        r#"rate(container_cpu_usage_seconds_total{container_label_com_docker_compose_service=~"service-[0-9]+"}[30s])*100"#
    }
    fn as_key(&self) -> &'static str {
        "CPU"
    }
    fn is_required(&self) -> bool {
        true
    }
}

struct PrometheusMemory;
impl PrometheusMetric for PrometheusMemory {
    fn to_string(&self) -> &'static str {
        r#"container_memory_working_set_bytes{container_label_com_docker_compose_service=~"service-[0-9]+"}/1e6"#
    }
    fn as_key(&self) -> &'static str {
        "MEMORY"
    }
    fn is_required(&self) -> bool {
        true
    }
}

struct PrometheusNetworkReceive;
impl PrometheusMetric for PrometheusNetworkReceive {
    fn to_string(&self) -> &'static str {
        r#"rate(container_network_receive_bytes_total{container_label_com_docker_compose_service=~"service-[0-9]+"}[30s])/1e6"#
    }
    fn as_key(&self) -> &'static str {
        "NETWORK_RECEIVE"
    }
    fn is_required(&self) -> bool {
        true
    }
}

struct PrometheusNetworkTransmit;
impl PrometheusMetric for PrometheusNetworkTransmit {
    fn to_string(&self) -> &'static str {
        r#"rate(container_network_transmit_bytes_total{container_label_com_docker_compose_service=~"service-[0-9]+"}[30s])/1e6"#
    }
    fn as_key(&self) -> &'static str {
        "NETWORK_TRANSMIT"
    }
    fn is_required(&self) -> bool {
        true
    }
}

struct PrometheusDiskRead;
impl PrometheusMetric for PrometheusDiskRead {
    fn to_string(&self) -> &'static str {
        r#"rate(container_fs_reads_bytes_total{container_label_com_docker_compose_service=~"service-[0-9]+"}[30s])/1e6"#
    }
    fn as_key(&self) -> &'static str {
        "DISK_READ"
    }
    fn is_required(&self) -> bool {
        false
    }
}

struct PrometheusDiskWrite;
impl PrometheusMetric for PrometheusDiskWrite {
    fn to_string(&self) -> &'static str {
        r#"rate(container_fs_writes_bytes_total{container_label_com_docker_compose_service=~"service-[0-9]+"}[30s])/1e6"#
    }
    fn as_key(&self) -> &'static str {
        "DISK_WRITE"
    }
    fn is_required(&self) -> bool {
        false
    }
}

lazy_static::lazy_static! {
    static ref METRICS: [Box<dyn PrometheusMetric + Sync>; 6] = [
        Box::new(PrometheusCPU{}),
        Box::new(PrometheusMemory{}),
        Box::new(PrometheusNetworkReceive{}),
        Box::new(PrometheusNetworkTransmit{}),
        Box::new(PrometheusDiskRead{}),
        Box::new(PrometheusDiskWrite{}),
    ];
}

pub async fn test_load_level(
    dir: impl AsRef<std::path::Path>,
    duration: usize,
) -> Result<HashMap<&'static str, f64>> {
    let mut iterations = tokio::fs::read_dir(dir.as_ref()).await?;
    let client = reqwest::Client::new();
    let mut results: HashMap<&'static str, Vec<Array1<f64>>> = HashMap::new();
    while let Some(iteration) = iterations.next_entry().await? {
        let path = iteration.path();
        let summary_file = tokio::fs::File::open(path.join("summary_out.csv")).await?;
        let mut first_line = String::new();
        let mut rdr = tokio::io::BufReader::new(summary_file);
        rdr.read_line(&mut first_line).await?;
        let (_, time) = first_line.rsplit_once(",").expect("comma");
        const TIME_STR_LEN: usize = "01.01.1970;00:00:00".len();
        log::debug!("Expected time str len: {TIME_STR_LEN}");
        let time = &time[..TIME_STR_LEN];
        let dt = NaiveDateTime::parse_from_str(time, "%d.%m.%Y;%H:%M:%S")
            .expect("date time")
            .and_utc();
        let start = dt.timestamp();
        log::debug!("Start timestamp: {start}");
        let end = (dt + Duration::seconds(duration as i64)).timestamp();
        log::debug!("End timestamp: {end}");
        let abs_path = tokio::fs::canonicalize(&path.join("metrics")).await?;
        let vol_map = format!("{}:/prometheus:rw", path_to_str(&abs_path)?);
        let mut docker = std::process::Command::new("docker");
        docker.args([
            "run",
            "-v",
            vol_map.as_str(),
            "-p",
            "9090:9090",
            "-e",
            "TZ=UTC",
            "--rm",
            "--user",
            "0:0",
            "--detach",
            "prom/prometheus:v2.49.1",
        ]);
        let cid = docker.output().expect("docker start");
        assert!(cid.status.success());
        let mut started = false;
        let mut sleep = 1;
        while !started {
            match client.get("http://localhost:9090/status").send().await {
                Ok(response) => {
                    if response.status() == 200 {
                        started = true;
                    }
                }
                Err(_) => {
                    log::info!("Waiting for prometheus server to start...");
                    tokio::time::sleep(std::time::Duration::new(sleep, 0)).await;
                    sleep += sleep;
                }
            }
        }
        for metric in METRICS.iter() {
            log::debug!("Pulling values for metric {}", metric.as_key());
            let request = client
                .post("http://localhost:9090/api/v1/query_range")
                .form(&[
                    ("query", metric.to_string()),
                    ("start", &start.to_string()),
                    ("end", &end.to_string()),
                    ("step", "1s"),
                ])
                .build()
                .unwrap();
            let mut retries = 3;
            let response = loop {
                if retries == 0 {
                    break None;
                }
                match client
                    .execute(request.try_clone().expect("Failed to clone request"))
                    .await
                {
                    Ok(response) => break Some(response),
                    Err(e) => {
                        eprintln!(
                            "Request failed: {}. Retrying... ({} retries left)",
                            e,
                            retries - 1
                        );
                        retries -= 1;
                        if retries > 0 {
                            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                        }
                    }
                }
            };
            let response = response.expect("Maximum retries for API request");
            let mut data: PrometheusAPIResponse = response.json().await.unwrap();
            assert_eq!(data.status, "success");
            if metric.is_required() {
                assert!(
                    !data.data.result.is_empty(),
                    "empty data result for {}",
                    metric.as_key()
                );
            } else if data.data.result.is_empty() {
                let mut values = Vec::with_capacity(((end - start) as usize) + 1);
                for ts in start..=end {
                    values.push((ts, "0.0".into()))
                }
                data.data.result.push(PrometheusAPIDataResult { values });
            }
            let mut values: HashMap<_, f64> = HashMap::default();
            data.data.result.iter().for_each(|result| {
                result.values.iter().for_each(|(ts, val)| {
                    let entry = values.entry(ts).or_default();
                    *entry += val.parse::<f64>().unwrap();
                });
            });
            let vec = results.entry(metric.as_key()).or_default();
            vec.push(Array1::from_iter(values.values().copied()));
        }
        let mut docker = std::process::Command::new("docker");
        let encoder = local_encoding::posix::EncoderUtf8 {};
        let cid = encoder.to_string(cid.stdout.as_slice()).unwrap();
        docker.arg("stop").arg(cid.trim());
        docker.output().expect("docker stop");
    }
    for (key, values) in results.iter() {
        if *key == PrometheusCPU.as_key()
            || *key == PrometheusNetworkReceive.as_key()
            || *key == PrometheusNetworkTransmit.as_key()
        {
            if values.iter().any(|vals| vals.iter().any(|v| *v != 0.0)) {
                let test = levene_test(values.as_slice());
                if test.p_value < 0.05 {
                    log::error!(
                        "detected unstable benchmark for path `{}` and metric {} (p={}<0.05)",
                        dir.as_ref().display(),
                        key,
                        test.p_value
                    );
                }
            } else {
                log::info!(
                    "All zero values for metric `{}` of path `{}`. Skipping Levene test...",
                    key,
                    dir.as_ref().display(),
                )
            }
        }
    }

    let averages: HashMap<&str, f64> = HashMap::from_iter(results.iter().map(|(key, values)| {
        (*key, {
            let (n_total, total_sum) = values.iter().map(|arr| (arr.len(), arr.sum())).fold(
                (0, 0.0),
                |mut acc, (len, sum)| {
                    acc.0 += len;
                    acc.1 += sum;
                    acc
                },
            );
            total_sum / n_total as f64
        })
    }));

    Ok(averages)
}
