use crate::{cli, Result};

pub async fn pull(args: &cli::profile::pull::Config) -> Result<()> {
    let profile_dir = std::path::PathBuf::from_iter([creo_lib::PROFILE_DIR, &args.app_name]);
    creo_lib::ssh::pull(
        &args.ssh_config,
        &profile_dir,
        std::path::Path::new(creo_lib::HANDLER_FUNCTION_DIR),
    )
    .await?;

    Ok(())
}
