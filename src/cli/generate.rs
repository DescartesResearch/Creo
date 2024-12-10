#[derive(argh::FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "generate")]
/// Generate a microservice application with the specified topology and resource usage profiles.
pub struct Command {
    #[argh(option, default = "std::path::PathBuf::from(\"config/generate.yml\")")]
    /// the path to the generation config file (Default: 'config/generate.yml')
    pub config: std::path::PathBuf,
}
