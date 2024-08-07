use crate::{cli, Result};

pub async fn benchmark(args: &cli::profile::BenchmarkConfig) -> Result<()> {
    let profile_dir = std::path::PathBuf::from(&args.profiling_application);

    creo_lib::ssh::benchmark(&args.ssh_config, &args.benchmark_config, profile_dir).await?;

    Ok(())
}
