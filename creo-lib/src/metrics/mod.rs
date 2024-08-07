mod cadvisor;
mod prometheus;

pub fn add_metrics_collection(
    dir: impl AsRef<std::path::Path>,
    depends_on: Vec<String>,
    compose: &mut crate::compose::Compose,
) -> std::io::Result<()> {
    let cadvisor = cadvisor::create_cadvisor_service(depends_on);
    let prometheus = prometheus::create_prometheus_service();
    compose.0.services.0.insert(cadvisor.0, Some(cadvisor.1));
    compose
        .0
        .services
        .0
        .insert(prometheus.0, Some(prometheus.1));
    let config = prometheus::Config {
        scrape_configs: vec![prometheus::ScrapeConfig {
            job_name: "cadvisor".into(),
            scrape_interval: "5s".into(),
            static_configs: vec![prometheus::StaticConfig {
                targets: vec!["cadvisor:8080".into()],
            }],
        }],
    };
    serde_yaml::to_writer(
        std::fs::File::create(dir.as_ref().join("prometheus.yml"))?,
        &config,
    )
    .map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "failed to write prometheus config to path {}!\n\tReason: {}",
                dir.as_ref().display(),
                err
            ),
        )
    })?;

    Ok(())
}
