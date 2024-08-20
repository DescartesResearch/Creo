use crate::{cli, Result};

pub async fn benchmark(args: &cli::profile::benchmark::Config) -> Result<()> {
    let profile_dir = std::path::PathBuf::from(&args.app_name);

    creo_lib::ssh::benchmark(&args.ssh_config, &args.benchmark_config, profile_dir).await?;

    Ok(())
}
