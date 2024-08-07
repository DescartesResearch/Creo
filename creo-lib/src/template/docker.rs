#[derive(serde::Serialize, Debug)]
pub struct DockerfileData {
    pub entrypoint: &'static str,
}
