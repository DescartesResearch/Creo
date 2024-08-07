use docker_compose_types as dct;

pub fn create_cadvisor_service(depends_on: Vec<String>) -> (String, dct::Service) {
    (
        "cadvisor".into(),
        dct::Service {
            image: Some("gcr.io/cadvisor/cadvisor:v0.47.2".into()),
            ports: dct::Ports::Short(vec!["5050:8080".into()]),
            expose: vec!["8080".into()],
            volumes: vec![
                dct::Volumes::Simple("/:/rootfs:ro".into()),
                dct::Volumes::Simple("/var/run:/var/run:rw".into()),
                dct::Volumes::Simple("/sys:/sys:ro".into()),
                dct::Volumes::Simple("/var/lib/docker/:/var/lib/docker:ro".into()),
                dct::Volumes::Simple("/dev:/dev:ro".into()),
            ],
            privileged: true,
            devices: vec!["/dev/kmsg".into()],
            depends_on: dct::DependsOnOptions::Simple(depends_on),
            ..Default::default()
        },
    )
}
