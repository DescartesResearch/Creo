use crate::{cli, Result};

pub async fn pull(args: &cli::profile::PullConfig) -> Result<()> {
    let profile_dir = std::path::PathBuf::from(&args.profiling_application);
    creo_lib::ssh::pull(&args.ssh_config, &profile_dir, &args.handler_dir).await?;

    Ok(())
}
