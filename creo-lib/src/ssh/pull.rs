use futures::{
    future::{try_join, try_join_all},
    TryFutureExt,
};
use tokio::io::AsyncWriteExt;

use super::{establish_connections, path_to_str, Client, Config, Error, Result};

pub async fn pull(
    ssh_config: &Config,
    profile_dir: impl AsRef<std::path::Path>,
    handler_dir: impl AsRef<std::path::Path>,
) -> Result<()> {
    let profile_dir = profile_dir.as_ref();
    let remote_app_root = std::path::Path::new(profile_dir.file_name().ok_or_else(|| {
        Error::InvalidArgument(format!(
            "expected a path to a named directory of the profiling application, but was {}",
            profile_dir.display()
        ))
    })?);
    let master_clients = establish_connections(ssh_config, ssh_config.master_hosts.iter()).await?;
    let remote_app_root_path = path_to_str(&remote_app_root)?;
    let lang_dir = handler_dir.as_ref().join(
        remote_app_root_path
            .strip_prefix("profile-")
            .expect("profile- prefix"),
    );
    let lang_dir = &lang_dir;
    log::info!("Starting downloading benchmark measurements!");
    try_join_all(master_clients.iter().map(|client| async move {
        for worker_dir in client.read_dir(remote_app_root_path).await? {
            if worker_dir.file_type().is_dir() {
                for dir in client
                    .read_dir(path_to_str(&remote_app_root.join(worker_dir.file_name()))?)
                    .await?
                {
                    if dir.file_type().is_dir() {
                        let service_path = remote_app_root
                            .join(worker_dir.file_name())
                            .join(dir.file_name());

                        let benchmark_path = service_path
                            .join("benchmarks");
                        let handler_dir = lang_dir
                            .join(
                                dir.file_name()
                                    .strip_prefix("handler-")
                                    .expect("handler- prefix"),
                            );
                        let local_benchmark_path = handler_dir
                            .join("benchmarks");
                        if local_benchmark_path.is_dir() {
                            log::info!(
                                "Removing already exisiting path `{}`",
                                local_benchmark_path.display()
                            );
                            tokio::fs::remove_dir_all(&local_benchmark_path).await?;
                        }
                        if !client.try_exists(path_to_str(&benchmark_path)?).await? {
                            log::warn!("Path `{}` does not exist for `{}`", benchmark_path.display(), client.get_connection_ip());
                            continue;
                        }
                        client.execute(format!("cd {}; tar -czf benchmarks.tar.gz benchmarks", service_path.display())).await?;
                        let archive_path = handler_dir.join("benchmarks.tar.gz");
                        create_and_download_file(client, service_path.join("benchmarks.tar.gz"), &archive_path).await?;
                        let tar_gz = std::fs::File::open(&archive_path)?;
                        let tar = flate2::read::GzDecoder::new(&tar_gz);
                        let mut archive = tar::Archive::new(tar);
                        match archive.unpack(&handler_dir) {
                            Ok(_) => (),
                            Err(err) => {
                                log::warn!("failed to unpack archive into {}", handler_dir.display());
                                log::error!("{}", err.to_string());
                                continue;
                            },
                        }
                        tokio::fs::remove_file(&archive_path).await?;

                        let mut benchmark_entires = tokio::fs::read_dir(&local_benchmark_path).await?;
                        let mut n_checked = 0;
                        while let Some(intensity_dir) = benchmark_entires.next_entry().await? {
                            if intensity_dir.file_type().await?.is_dir() {
                                let mut intensity_entries = tokio::fs::read_dir(intensity_dir.path()).await?;
                                while let Some(run_dir) = intensity_entries.next_entry().await? {
                                    if run_dir.file_type().await?.is_dir() {
                                        let summary_file_path = run_dir.path().join("summary_out.csv");
                                        if let Ok(mut r) = csv::ReaderBuilder::new().flexible(true).from_path(&summary_file_path) {
                                            let mut n_transactions = 0;
                                            let mut n_failed_transactions = 0;
                                            let mut n_dropped_transactions = 0;
                                            let mut record = csv::StringRecord::new();
                                            while r.read_record(&mut record).unwrap() {
                                                n_transactions += record.get(1).unwrap().parse::<f64>().unwrap() as usize;
                                                n_failed_transactions += record.get(3).unwrap().parse::<usize>().unwrap();
                                                n_dropped_transactions += record.get(4).unwrap().parse::<usize>().unwrap();
                                            }
                                            if n_dropped_transactions > ((0.01 * n_transactions as f64) as usize) {
                                                log::warn!("AssertionFailed for `{}`", summary_file_path.display());
                                                log::warn!("Dropped more than 1% ({} > 0.01 * {}) of transactions...", n_dropped_transactions, n_transactions);
                                            }
                                            if n_failed_transactions > ((0.01 * n_transactions as f64) as usize) {
                                                log::warn!("AssertionFailed for `{}`", summary_file_path.display());
                                                log::warn!("Failed more than 1% ({} > 0.01 * {}) of transactions...", n_failed_transactions, n_transactions);
                                            }
                                        } else {
                                            log::warn!("File `{}` does not exist...", summary_file_path.display());
                                        }
                                        n_checked += 1;
                                    }
                                }
                            }
                        }
                        log::info!("Checked {} directories for `{}`", n_checked, local_benchmark_path.display());
                    }
                }
            }
        }
        Ok::<(), Error>(())
    }))
    .await?;
    log::info!("Finished downloading benchmark measurements!");

    Ok(())
}

pub async fn create_and_download_file(
    client: &Client,
    remote_file: impl AsRef<std::path::Path>,
    local_file: impl AsRef<std::path::Path>,
) -> Result<()> {
    let (mut local_file, remote_file_content) = try_join(
        tokio::fs::File::create(&local_file).map_err(Error::from),
        client.read(path_to_str(&remote_file)?),
    )
    .await?;
    local_file.write_all(remote_file_content.as_slice()).await?;
    Ok(())
}
