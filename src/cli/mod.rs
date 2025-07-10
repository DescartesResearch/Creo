pub mod benchmark;
pub mod deploy;
pub mod download;
pub mod generate;
pub mod profile;

use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum Commands {
    Generate(generate::Command),
    Profile(profile::Command),
    Deploy(deploy::Command),
    Benchmark(benchmark::Command),
    Download(download::Command),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Creo generates microservice applications for performance benchmarking.
pub struct Args {
    #[argh(subcommand)]
    pub command: Option<Commands>,
    #[argh(switch, short = 'v')]
    /// display the version
    pub version: bool,

    #[argh(option, short = 'o')]
    /// output directory
    pub output: Option<std::path::PathBuf>,
}
