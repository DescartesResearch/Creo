use crate::{cli, Result};

pub async fn aggregate(args: &cli::profile::AggregateConfig) -> Result<()> {
    let lang_dir = args
        .aggregate
        .handlers
        .join(args.programming_language.as_dir_name());

    creo_lib::ssh::aggregate(&args.benchmark_config, &lang_dir).await?;

    Ok(())
}
