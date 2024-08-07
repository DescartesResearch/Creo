use docker_compose_types as dct;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MongoDB;

impl MongoDB {
    const MONGO_USER: &'static str = "user";
    const MONGO_PASSWORD: &'static str = "sup3rs3cr3t";
    const MONGO_PORT: &'static str = "27017";

    pub(super) fn as_docker_compose_service(
        service_name: impl AsRef<str>,
    ) -> (String, dct::Service) {
        (
            Self::as_service_name(&service_name),
            dct::Service {
                image: Some("mongo:7.0.5".into()),
                environment: dct::Environment::List(vec![
                    Self::as_user_environment("MONGO_INITDB_ROOT_USERNAME"),
                    Self::as_password_environment("MONGO_INITDB_ROOT_PASSWORD"),
                ]),
                expose: vec![Self::MONGO_PORT.into()],
                restart: Some("unless-stopped".into()),
                healthcheck: Some(dct::Healthcheck {
                    test: Some(dct::HealthcheckTest::Single("echo 'db.runCommand({serverStatus:1}).ok' | mongosh admin -u $$MONGO_INITDB_ROOT_USERNAME -p $$MONGO_INITDB_ROOT_PASSWORD --quiet | grep 1".into())),
                    interval: Some("10s".into()),
                    timeout: Some("10s".into()),
                    retries: 5,
                    start_period: Some("40s".into()),
                    disable: false
                }),
                volumes: vec![dct::Volumes::Simple(format!("{}:/data/db", Self::as_volume_name(&service_name)))],
                ..Default::default()
            },
        )
    }

    pub(super) fn as_volume_name(service_name: impl AsRef<str>) -> String {
        format!("data-{}", Self::as_service_name(service_name))
    }

    pub fn as_service_name(service_name: impl AsRef<str>) -> String {
        format!("mongo-db-{}", service_name.as_ref())
    }

    fn as_user_environment(key: &str) -> String {
        format!("{}={}", key, Self::MONGO_USER)
    }

    fn as_password_environment(key: &str) -> String {
        format!("{}={}", key, Self::MONGO_PASSWORD)
    }

    pub(super) fn as_docker_compose_environment(service_name: impl AsRef<str>) -> Vec<String> {
        vec![
            format!("DB_MONGO_HOST={}", Self::as_service_name(service_name)),
            format!("DB_MONGO_PORT={}", Self::MONGO_PORT),
            Self::as_user_environment("DB_MONGO_USER"),
            Self::as_password_environment("DB_MONGO_PASSWORD"),
        ]
    }
}
