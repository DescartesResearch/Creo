pub mod aggregate;
pub mod benchmark;
pub mod deploy;
pub mod generate;
pub mod pull;

#[derive(argh::FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "profile")]
/// Generate, deploy, and benchmark profiling applications for a specific language to obtain
/// resource labels for the language's handler functions.
pub struct Command {
    #[argh(subcommand)]
    pub command: ProfileSubCommands,
}

#[derive(argh::FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
/// Subcommands for `profile`.
pub enum ProfileSubCommands {
    Generate(generate::SubCommand),
    Deploy(deploy::SubCommand),
    Benchmark(benchmark::SubCommand),
    Pull(pull::SubCommand),
    Aggregate(aggregate::SubCommand),
}
