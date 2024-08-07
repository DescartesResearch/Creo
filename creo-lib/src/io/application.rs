pub fn create_application_directory(
    path: impl AsRef<std::path::Path>,
    meta_data: ApplicationMetaData<'_>,
) -> std::io::Result<()> {
    crate::io::create_dir_all(&path)?;

    write_application_meta_data(path.as_ref(), meta_data)?;

    //TODO: docker-compose.yml, load_generator.yml, prometheus.yml, user_request.yml

    Ok(())
}

fn write_application_meta_data(
    app_dir: impl AsRef<std::path::Path>,
    meta_data: ApplicationMetaData<'_>,
) -> std::io::Result<()> {
    let file = app_dir.as_ref().join("META_DATA.json");
    let file = std::fs::File::create(&file)?;
    serde_json::to_writer_pretty(file, &meta_data)?;

    Ok(())
}

#[derive(serde::Serialize)]
pub struct ApplicationMetaData<'a> {
    pub application_name: &'a str,
    pub seed: &'a str,
    pub ports: Ports,
}

#[derive(serde::Serialize)]
pub struct Ports {
    pub start: u32,
    pub end: u32,
}
