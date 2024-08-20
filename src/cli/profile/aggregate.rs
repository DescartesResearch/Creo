#[derive(argh::FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "aggregate")]
/// Aggregate the pulled benchmark results for each handler function of the profiling application
/// into the corresponding resource labels.
pub struct SubCommand {
    #[argh(option, default = "std::path::PathBuf::from(\"config/profile.yml\")")]
    /// the path to the profiling application configuration file.
    pub config: std::path::PathBuf,
}

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    /// benchmark configuration (e.g., number of iterations, workload)
    #[serde(alias = "benchmark")]
    pub benchmark_config: creo_lib::ssh::BenchmarkConfig,

    pub programming_language: creo_lib::programming_lanuage::ProgrammingLanguage,
}
