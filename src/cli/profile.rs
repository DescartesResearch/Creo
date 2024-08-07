use std::ffi::OsString;

use creo_lib::programming_lanuage::ProgrammingLanguage;

#[derive(argh::FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "profile")]
/// Profile related commands
pub struct ProfileCommand {
    #[argh(subcommand)]
    pub command: ProfileCommands,
}

#[derive(argh::FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum ProfileCommands {
    Generate(GenerateCommand),
    Deploy(DeployCommand),
    Benchmark(BenchmarkCommand),
    Pull(PullCommand),
    Aggregate(AggregateCommand),
}

#[derive(Debug, serde::Deserialize)]
pub struct GenerateConfig {
    /// the programming language of the handler functions to profile
    #[serde(alias = "programming_language")]
    pub language: ProgrammingLanguage,

    /// the path to the handler directory. If the given path is relative, it will be relative to
    /// the current working directory.
    #[serde(default = "default_handler_dir", alias = "handlers")]
    pub handler_dir: OsString,

    /// the path to the templates directory. If the given path is relative, it will be relative to
    /// the current working directory.
    #[serde(default = "default_templates_dir", alias = "templates")]
    pub templates_dir: OsString,

    /// starting port published by the generated services. Gets incremented for each service.
    /// (Default = 30100)
    #[serde(alias = "start_port", default = "default_start_port")]
    pub start_port: u32,
}

fn default_handler_dir() -> OsString {
    OsString::from("assets/handlers/")
}

fn default_templates_dir() -> OsString {
    OsString::from("assets/templates/")
}

fn default_start_port() -> u32 {
    30100
}

#[derive(argh::FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "generate")]
/// The `generate` command allows profiling handler functions of a given language in order to obtain
/// usage metrics.
pub struct GenerateCommand {
    #[argh(
        option,
        default = "std::path::PathBuf::from(\"config/profile.yml\")",
        long = "handlers"
    )]
    /// the path to the config file. If the given path is relative, it will be relative to the
    /// current working directory.
    pub config: std::path::PathBuf,
}

#[derive(Debug, serde::Deserialize)]
#[serde(default)]
pub struct DeployConfig {
    /// the path to the directory containing all load generator related files. If the given path is
    /// relative, it will be relative to the current working directory (Defaults to:
    /// "assets/load_generator")
    #[serde(default = "default_load_generator")]
    pub load_generator: String,
}

impl Default for DeployConfig {
    fn default() -> Self {
        Self {
            load_generator: default_load_generator(),
        }
    }
}

fn default_load_generator() -> String {
    String::from("assets/load_generator")
}

#[derive(Debug, serde::Deserialize)]
pub struct ProfileDeployConfig {
    #[serde(default)]
    pub deploy: DeployConfig,
    /// the path to the deploy configuration file. If the given path is relative, it will be
    /// relative to the current working directory
    pub ssh_config: creo_lib::ssh::Config,
    /// the path to the profiling application directory. If the given path is relative, it will be
    /// relative to the current working directory
    #[serde(alias = "application")]
    pub profiling_application: String,
}

#[derive(argh::FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "deploy")]
/// The `deploy` command allows deploying the generated profiling services in order to enable
/// running profiling benchmarks.
pub struct DeployCommand {
    #[argh(
        option,
        default = "std::path::PathBuf::from(\"config/profile.yml\")",
        long = "config"
    )]
    /// the path to the config file. If the given path is relative, it will be relative to the
    /// current working directory.
    pub config: std::path::PathBuf,
}

#[derive(Debug, serde::Deserialize)]
pub struct BenchmarkConfig {
    /// the path to the benchmark configuration file. If the given path is relative, it will be
    /// relative to the current working directory
    #[serde(alias = "benchmark")]
    pub benchmark_config: creo_lib::ssh::BenchmarkConfig,

    /// the path to the deploy configuration file. If the given path is relative, it will be
    /// relative to the current working directory
    pub ssh_config: creo_lib::ssh::Config,

    /// the path to the profiling application directory. If the given path is relative, it will be
    /// relative to the current working directory
    #[serde(alias = "application")]
    pub profiling_application: String,
}

#[derive(argh::FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "benchmark")]
/// The `benchmark` command allows starting the profiling benchmarks.
pub struct BenchmarkCommand {
    #[argh(
        option,
        default = "std::path::PathBuf::from(\"config/profile.yml\")",
        long = "handlers"
    )]
    /// the path to the config file. If the given path is relative, it will be relative to the
    /// current working directory.
    pub config: std::path::PathBuf,
}

#[derive(Debug, serde::Deserialize)]
pub struct PullConfig {
    /// the path to the deploy configuration file. If the given path is relative, it will be
    /// relative to the current working directory
    pub ssh_config: creo_lib::ssh::Config,

    /// the path to the profiling application directory. If the given path is relative, it will be
    /// relative to the current working directory
    #[serde(alias = "application")]
    pub profiling_application: String,

    #[serde(default = "default_handler_path", alias = "handlers")]
    /// the path to the handler directory. If the given path is relative, it will be relative to
    /// the current working directory.
    pub handler_dir: std::path::PathBuf,
}

fn default_handler_path() -> std::path::PathBuf {
    std::path::PathBuf::from("assets/handlers/")
}

#[derive(argh::FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "pull")]
/// The `pull` command enables pulling the profiling benchmark results. In addition, the results
/// are aggregated into a single value per resource metric. This creates the corresponding
/// [`utilization.yml`] file for each profiled handler.
pub struct PullCommand {
    #[argh(
        option,
        default = "std::path::PathBuf::from(\"config/profile.yml\")",
        long = "handlers"
    )]
    /// the path to the config file. If the given path is relative, it will be relative to the
    /// current working directory.
    pub config: std::path::PathBuf,
}

#[derive(Debug, serde::Deserialize)]
pub struct AggregateLangConfig {
    /// the path to the handler directory. If the given path is relative, it will be relative to
    /// the current working directory.
    pub handlers: std::path::PathBuf,
}

impl Default for AggregateLangConfig {
    fn default() -> Self {
        Self {
            handlers: default_handler_path(),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct AggregateConfig {
    /// the path to the benchmark configuration file. If the given path is relative, it will be
    /// relative to the current working directory
    #[serde(alias = "benchmark")]
    pub benchmark_config: creo_lib::ssh::BenchmarkConfig,

    #[serde(default)]
    pub aggregate: AggregateLangConfig,

    pub programming_language: ProgrammingLanguage,
}

#[derive(argh::FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "aggregate")]
/// The `aggregate` command enables aggregating the profiling benchmark results.
/// This creates the corresponding [`utilization.yml`] file for each profiled handler.
pub struct AggregateCommand {
    #[argh(
        option,
        default = "std::path::PathBuf::from(\"config/profile.yml\")",
        long = "handlers"
    )]
    /// the path to the config file. If the given path is relative, it will be relative to the
    /// current working directory.
    pub config: std::path::PathBuf,
}
