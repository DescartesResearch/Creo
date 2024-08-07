pub mod benchmark;
pub mod deploy;
pub mod download;
pub mod generate;
pub mod profile;

use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum Commands {
    Generate(generate::GenerateCommand),
    Profile(profile::ProfileCommand),
    Deploy(deploy::Command),
    Benchmark(benchmark::Command),
    Download(download::Command),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Creo enables generating microservices for performance benchmarking
pub struct Args {
    #[argh(subcommand)]
    pub command: Option<Commands>,
    #[argh(switch, short = 'v', description = "output the version")]
    pub version: bool,
}
