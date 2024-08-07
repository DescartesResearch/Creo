pub fn write_docker_compose_file(
    file: impl AsRef<std::path::Path>,
    docker_compose: &crate::compose::Compose,
) -> std::io::Result<()> {
    serde_yaml::to_writer(std::fs::File::create(file.as_ref())?, &docker_compose.0).map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "failed to write docker-compose.yml file for path {}!\n\tReason: {}",
                file.as_ref().display(),
                err
            ),
        )
    })
}
