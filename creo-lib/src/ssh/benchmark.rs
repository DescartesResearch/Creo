use futures::future::try_join_all;
use tokio::io::AsyncWriteExt;

use crate::template::register_template_file;

use super::{config::BenchmarkConfig, establish_connections, path_to_str, Config, Error, Result};

pub async fn benchmark(
    ssh_config: &Config,
    benchmark_config: &BenchmarkConfig,
    profile_dir: impl AsRef<std::path::Path>,
) -> Result<()> {
    let profile_dir = profile_dir.as_ref();
    let remote_app_root = std::path::Path::new(profile_dir.file_name().ok_or_else(|| {
        Error::InvalidArgument(format!(
            "expected a path to a named directory of the profiling application, but was {}",
            profile_dir.display()
        ))
    })?);
    let master_clients = establish_connections(ssh_config, ssh_config.master_hosts.iter()).await?;
    let mut template_path = std::path::PathBuf::from(crate::SCRIPTS_DIR);
    template_path.push("start.mgt");
    let template = register_template_file("start", &template_path)?;

    try_join_all(master_clients.iter().map(|client| {
        let template = &template;
        let template_data = benchmark_config;
        async move {
            let script_path = remote_app_root.join("start.sh");
            let remote_script_path = path_to_str(&script_path)?;
            let config_path = remote_app_root.join("config.yml");
            let remote_config_path = path_to_str(&config_path)?;

            create_file_with_content(
                client,
                remote_script_path.to_string(),
                template.render("start", template_data)?.as_bytes(),
            )
            .await?;
            create_file_with_content(client,
                remote_config_path.to_string(),
                serde_yaml::to_string(benchmark_config)?.as_bytes(),
            ).await?;
            for dir in client.read_dir(path_to_str(&remote_app_root)?).await? {
                if dir.file_type().is_dir() {

                    let screen_name = format!("{}-{}", path_to_str(&remote_app_root)?, dir.file_name());
                    let command = format!(
                        "screen -dm -S {} -L -Logfile {}.log ./{} {}",
                        screen_name,
                        screen_name,
                        remote_script_path,
                        dir.file_name(),
                    );
                    client
                        .execute(command)
                    .await?;
                    log::info!("\n\nStarted benchmark runs for host `{}`\nUse the following command on the server to observe benchmark progress:\n\n\tscreen -r {}\n\n", client.get_connection_ip(), screen_name);
                }
            }


            Ok::<(), Error>(())
        }
    }))
    .await?;

    Ok(())
}

pub async fn create_file_with_content(
    client: &super::Client,
    remote_file_path: String,
    content: &[u8],
) -> Result<russh_sftp::client::fs::File> {
    let mut remote_file = client.create(&remote_file_path).await?;
    remote_file.write_all(content).await?;
    Ok(remote_file)
}
