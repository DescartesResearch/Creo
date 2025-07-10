pub async fn invoke(
    config: &creo_lib::ssh::Config,
    name: String,
    out: impl AsRef<std::path::Path>,
) -> crate::Result<()> {
    if config.master_hosts.len() != 1 {
        return Err(crate::Error::new(
            "application deployment only supports 1 master host at the moment".into(),
        ));
    }

    let master_clients =
        creo_lib::ssh::establish_connections(config, config.master_hosts.iter()).await?;
    let master_client = &master_clients[0];
    let remote_app_path = master_client.canonicalize(&name).await?;
    let remote_archive_path = format!("{}/benchmarks.tar.gz", &remote_app_path);

    master_client
        .execute(format!(
            r#"tar -czf "{}" -C "{}" benchmarks"#,
            &remote_archive_path, &remote_app_path
        ))
        .await?;

    let app_path = out.as_ref().join(&name);

    let local_archive_path = app_path.join("benchmarks.tar.gz");
    let mut local_archive = tokio::fs::File::create(&local_archive_path).await?;
    let mut remote_archive = master_client.open(remote_archive_path).await?;

    tokio::io::copy(&mut remote_archive, &mut local_archive).await?;

    creo_lib::remote::archive::extract_archive(&local_archive_path, app_path).await?;

    tokio::fs::remove_file(local_archive_path).await?;

    Ok(())
}
