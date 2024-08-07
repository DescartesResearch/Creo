use docker_compose_types as dct;

pub fn create_prometheus_service() -> (String, dct::Service) {
    (
        "prometheus".into(),
        dct::Service {
            image: Some("prom/prometheus:v2.49.1".into()),
            ports: dct::Ports::Short(vec!["9090:9090".into()]),
            // user: Some("65534:65534".into()),
            // command: Some(dct::Command::Simple(
            //     "--config.file=/etc/prometheus/prometheus.yml".into(),
            // )),
            user: Some(
                "${PROMETHEUS_UID?PROMETHEUS_UID unset}:${PROMETHEUS_GID?PROMETHEUS_GID unset}"
                    .into(),
            ),
            volumes: vec![
                dct::Volumes::Simple(
                    "$PWD/prometheus.yml:/etc/prometheus/prometheus.yml:ro".into(),
                ),
                dct::Volumes::Simple("${PWD}/metrics:/prometheus:rw".into()),
            ],
            depends_on: dct::DependsOnOptions::Simple(vec!["cadvisor".into()]),
            ..Default::default()
        },
    )
}

#[derive(serde::Serialize, Debug)]
pub struct Config {
    pub scrape_configs: Vec<ScrapeConfig>,
}

#[derive(serde::Serialize, Debug)]
pub struct ScrapeConfig {
    pub job_name: String,
    pub scrape_interval: String,
    pub static_configs: Vec<StaticConfig>,
}

#[derive(serde::Serialize, Debug)]
pub struct StaticConfig {
    pub targets: Vec<String>,
}
