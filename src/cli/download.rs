#[derive(argh::FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "download")]
/// Download all experiment results for an application.
pub struct Command {
    #[argh(
        option,
        default = "std::path::PathBuf::from(\"config/benchmark.yml\")",
        long = "config"
    )]
    /// the path to the deployment configuration file
    pub config_path: std::path::PathBuf,
}

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    /// SSH configuration options
    pub ssh: creo_lib::ssh::Config,
    /// path to the application
    pub application: String,
}
