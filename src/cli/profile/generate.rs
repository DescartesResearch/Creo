#[derive(argh::FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "generate")]
/// Generate a profiling application for a specific programming language.
pub struct SubCommand {
    #[argh(option, default = "std::path::PathBuf::from(\"config/profile.yml\")")]
    /// the path to the profiling application configuration file.
    pub config: std::path::PathBuf,
}

#[derive(Debug, serde::Deserialize)]
/// Generation configuration of a profiling application for a specific programming language.
pub struct Config {
    /// the target programming language of the profiling application
    #[serde(alias = "programming_language")]
    pub language: creo_lib::programming_lanuage::ProgrammingLanguage,

    /// The port the generated profiling application start to publish on. The first service uses this port,
    /// while each subsequent service uses the next port number after the previous service's port. (Default: 30100)
    ///
    /// In other words, the generated profiling application occupies the port range starting from the
    /// specified port `p` up to the ending port number `p+h-1`, where `h` is the number of
    /// handler functions of the targeted language.
    #[serde(alias = "start_port", default = "default_start_port")]
    pub start_port: u32,
}

fn default_start_port() -> u32 {
    30100
}
