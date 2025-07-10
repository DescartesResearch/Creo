use testcontainers::core::WaitFor;
use testcontainers::core::wait::LogWaitStrategy;
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, GenericImage, ImageExt, core::IntoContainerPort};

pub async fn start_mysql() -> ContainerAsync<GenericImage> {
    GenericImage::new("mariadb", "11")
        .with_exposed_port(3306.tcp())
        .with_wait_for(WaitFor::log(
            LogWaitStrategy::stderr("ready for connections").with_times(2),
        ))
        .with_env_var("MARIADB_USER", "creo")
        .with_env_var("MARIADB_PASSWORD", "creopassword")
        .with_env_var("MARIADB_ROOT_PASSWORD", "creopassword")
        .with_env_var("MARIADB_DATABASE", "stats")
        .start()
        .await
        .expect("MariaDB to start")
}
