use std::collections::HashSet;

use creo_lib::programming_lanuage::ProgrammingLanguage;
use rand::Rng;
use serde::Deserialize;
use strum::IntoEnumIterator;

#[derive(Debug, serde::Deserialize)]
pub struct GenerateConfig {
    /// selects the given programming languages as available during generation (comma separated
    /// list). Allowed values: python
    #[serde(
        alias = "programming_languages",
        deserialize_with = "unique_languages",
        default = "select_all_programming_languages"
    )]
    pub selected_languages: Vec<ProgrammingLanguage>,

    /// the name of the generated microservice application
    pub application_name: String,

    /// the path to the handler directory. If the given path is relative, it will be relative to
    /// the current working directory.
    #[serde(default = "default_handler_dir", alias = "handlers")]
    pub handler_dir: std::path::PathBuf,

    /// the path to the templates directory. If the given path is relative, it will be relative to
    /// the current working directory.
    #[serde(default = "default_templates_dir", alias = "templates")]
    pub templates_dir: std::path::PathBuf,

    /// the number of endpoints (nodes) to generate
    #[serde(alias = "endpoints")]
    pub number_of_endpoints: usize,

    /// the number of service calls (edges) to generate
    #[serde(alias = "service_calls", default)]
    pub number_of_service_calls: usize,

    /// the number of services to generate
    #[serde(alias = "services")]
    pub number_of_services: usize,

    /// service type definitions
    #[serde(alias = "service_types")]
    pub service_types: creo_lib::ServiceTypeVec,

    /// set the random seed (defaults to a random, 16 characters long string)
    #[serde(default = "create_random_seed")]
    pub seed: String,

    /// starting port published by the generated services. Gets incremented for each service.
    /// (Default = 30100)
    #[serde(default = "default_start_port")]
    pub start_port: u32,

    #[serde(alias = "service_call_list", default)]
    pub service_call_list: Vec<(usize, usize)>,
}

fn default_handler_dir() -> std::path::PathBuf {
    std::path::PathBuf::from("assets/handlers/")
}

fn default_templates_dir() -> std::path::PathBuf {
    std::path::PathBuf::from("assets/templates/")
}

fn default_start_port() -> u32 {
    30100
}

#[derive(argh::FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "generate")]
/// The `generate` command allows generating microservices using the given parameters.
pub struct GenerateCommand {
    #[argh(
        option,
        default = "std::path::PathBuf::from(\"config/generate.yml\")",
        long = "handlers"
    )]
    /// the path to the config file. If the given path is relative, it will be relative to the
    /// current working directory.
    pub config: std::path::PathBuf,
}

fn select_all_programming_languages() -> Vec<ProgrammingLanguage> {
    ProgrammingLanguage::iter().collect()
}

fn unique_languages<'de, D>(deserializer: D) -> Result<Vec<ProgrammingLanguage>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let langs: Vec<ProgrammingLanguage> = Deserialize::deserialize(deserializer)?;
    let mut seen: HashSet<_> = HashSet::new();
    for lang in &langs {
        if !seen.insert(lang) {
            return Err(serde::de::Error::custom(format!(
                "duplicate programming language {}",
                lang
            )));
        }
    }
    let sum: usize = langs.iter().map(|l| l.as_fraction()).sum();
    if !langs.iter().all(|l| l.as_fraction() == 1) && sum != 100 {
        return Err(serde::de::Error::custom(format!(
            "expected programming language fractions to sum to 100, but was {}",
            sum
        )));
    }
    Ok(langs)
}

fn create_random_seed() -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(16)
        .map(char::from)
        .collect()
}
