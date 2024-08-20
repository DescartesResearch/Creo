use crate::{cli, Result};

pub async fn aggregate(args: &cli::profile::aggregate::Config) -> Result<()> {
    let lang_dir = std::path::PathBuf::from_iter([
        creo_lib::HANDLER_FUNCTION_DIR,
        args.programming_language.as_dir_name(),
    ]);

    creo_lib::ssh::aggregate(&args.benchmark_config, &lang_dir).await?;

    Ok(())
}
