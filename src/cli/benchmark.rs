#[derive(argh::FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "benchmark")]
/// Start a benchmark for a deployed microservice applicaton.
pub struct Command {
    #[argh(
        option,
        default = "std::path::PathBuf::from(\"config/benchmark.yml\")",
        long = "config"
    )]
    /// the path to the benchmark configuration file
    pub config: std::path::PathBuf,
}

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    /// SSH configuration options
    pub ssh: creo_lib::ssh::Config,
    /// application name
    pub application: String,
    /// benchmark configuration
    pub benchmark: Benchmark,
}

#[derive(Debug, serde::Deserialize)]
pub struct Benchmark {
    pub records: u64,
    pub virtual_user: u64,
    pub timeout: u64,
    pub warmup: Warmup,
    pub intensity: Intensity,
}

#[derive(Debug, serde::Deserialize)]
pub struct Warmup {
    pub rate: u64,
    pub duration: u64,
    pub pause: u64,
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum Intensity {
    PROFILE { profile: String },
}
