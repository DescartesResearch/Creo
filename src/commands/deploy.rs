use std::path::PathBuf;

pub async fn invoke(config: &creo_lib::ssh::Config, name: String) -> crate::Result<()> {
    if config.master_hosts.len() != 1 {
        return Err(crate::Error::new(
            "application deployment only supports 1 master host at the moment".into(),
        ));
    }
    if config.worker_hosts.len() != 1 {
        return Err(crate::Error::new(
            "application deployment only supports 1 worker host at the moment".into(),
        ));
    }

    let clients = creo_lib::ssh::establish_connections(
        config,
        config.master_hosts.iter().chain(config.worker_hosts.iter()),
    )
    .await?;
    let (master_clients, worker_clients) = clients.split_at(config.master_hosts.len());
    let master_client = &master_clients[0];
    let worker_client = &worker_clients[0];

    let src = std::path::PathBuf::from_iter(&[creo_lib::OUTPUT_DIR, &name]);

    let (remote_worker_path, remote_master_path) = tokio::try_join!(
        worker_client.canonicalize("archive.tar.gz"),
        master_client.canonicalize("archive.tar.gz")
    )?;

    let local_worker_archive_path = std::env::temp_dir().join("worker-archive.tar.gz");
    let local_worker_archive = tokio::fs::File::create(&local_worker_archive_path).await?;
    let mut worker_builder =
        creo_lib::remote::archive::Builder::new_compressed(local_worker_archive);
    worker_builder
        .append_dir_all(
            &name,
            &src,
            &[
                "**/benchmarks/",
                "**/metrics/",
                "**/user_requests.yml",
                "**/load_generator.yml",
            ],
        )
        .await?;
    worker_builder.into_inner().await?;

    let local_master_archive_path = std::env::temp_dir().join("master-archive.tar.gz");
    let local_master_archive = tokio::fs::File::create(&local_master_archive_path).await?;
    let mut master_builder =
        creo_lib::remote::archive::Builder::new_compressed(local_master_archive);
    let app_path: PathBuf = name.into();
    master_builder
        .append_file(
            app_path.join("user_requests.yml"),
            &mut tokio::fs::File::open(src.join("user_requests.yml")).await?,
        )
        .await?;
    master_builder
        .append_file(
            app_path.join("benchmark.sh"),
            &mut tokio::fs::File::open(PathBuf::from("assets/scripts/benchmark.sh")).await?,
        )
        .await?;
    master_builder
        .append_dir_all("load_generator", &creo_lib::LOAD_GENERATOR_DIR, &[])
        .await?;
    master_builder.into_inner().await?;

    creo_lib::remote::upload_and_extract_archive(
        worker_client,
        &remote_worker_path,
        local_worker_archive_path,
    )
    .await
    .map_err(|err| {
        crate::Error::new(format!(
            "failed to deploy application to host `{}`: {err}",
            worker_client.get_connection_ip()
        ))
    })?;

    creo_lib::remote::upload_and_extract_archive(
        master_client,
        &remote_master_path,
        local_master_archive_path,
    )
    .await
    .map_err(|err| {
        crate::Error::new(format!(
            "failed to deploy benchmarking harness to host `{}`: {err}",
            master_client.get_connection_ip()
        ))
    })?;

    Ok(())
}