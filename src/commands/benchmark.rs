pub async fn invoke(
    ssh_config: &creo_lib::ssh::Config,
    app_name: String,
    benchmark_config: &crate::cli::benchmark::Benchmark,
) -> crate::Result<()> {
    if ssh_config.master_hosts.len() != 1 {
        return Err(crate::Error::new(
            "benchmarking only supports 1 master host at the moment".into(),
        ));
    }
    if ssh_config.worker_hosts.len() != 1 {
        return Err(crate::Error::new(
            "benchmarking only supports 1 worker host at the moment".into(),
        ));
    }

    let client =
        creo_lib::ssh::establish_connections(ssh_config, ssh_config.master_hosts.iter()).await?;
    let client = &client[0];

    client
        .execute(format!(
            r#"sed 's/{{{{APPLICATION_HOST}}}}/{}/g' < "{}/user_requests.lua" > "{}/output.lua""#,
            ssh_config.worker_hosts[0], &app_name, &app_name
        ))
        .await?;

    match &benchmark_config.intensity {
        crate::cli::benchmark::Intensity::PROFILE { profile } => {
            client.execute(format!(r#"screen -dm -S "{}" -L -Logfile "{}/benchmark.log" "{}/benchmark.sh" {} {} {} {} {} {} {} {} {} {}"#, &app_name, &app_name, &app_name, &ssh_config.user_name, &app_name, &ssh_config.worker_hosts[0], benchmark_config.virtual_user, benchmark_config.timeout, benchmark_config.warmup.pause, benchmark_config.warmup.duration, benchmark_config.warmup.rate, benchmark_config.records, profile)).await?;
        }
    }

    log::info!("You can observe the benchmark progress on `{}` with the following command:\n\tscreen -r {}", client.get_connection_ip(), app_name);

    Ok(())
}
