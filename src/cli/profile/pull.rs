#[derive(argh::FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "pull")]
/// Pull the benchmark results for a specific profiling application to your local machine.
pub struct SubCommand {
    #[argh(option, default = "std::path::PathBuf::from(\"config/profile.yml\")")]
    /// the path to the profiling application configuration file.
    pub config: std::path::PathBuf,
}

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    /// ssh configuration (e.g., master- and worker-hosts)
    pub ssh_config: creo_lib::ssh::Config,

    /// the name of the profiling application
    #[serde(alias = "application")]
    pub app_name: String,
}
