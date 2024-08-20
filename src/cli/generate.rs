use std::collections::HashSet;

use creo_lib::programming_lanuage::ProgrammingLanguage;
use rand::Rng;
use serde::Deserialize;
use strum::IntoEnumIterator;

#[derive(argh::FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "generate")]
/// Generate a microservice application with the specified topology and resource usage profiles.
pub struct Command {
    #[argh(option, default = "std::path::PathBuf::from(\"config/generate.yml\")")]
    /// the path to the generation config file (Default: 'config/generate.yml')
    pub config: std::path::PathBuf,
}

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    /// The programming languages that are available during the generation.
    #[serde(
        alias = "programming_languages",
        deserialize_with = "deserialize_languages",
        default = "select_all_programming_languages"
    )]
    pub selected_languages: Vec<ProgrammingLanguage>,

    /// The application name.
    #[serde(alias = "application_name")]
    pub app_name: String,

    /// The number of endpoints (vertices) to generate.
    #[serde(alias = "endpoints")]
    pub number_of_endpoints: usize,

    /// The number of inter-service calls (edges) to generate.
    #[serde(alias = "service_calls", default)]
    pub number_of_service_calls: usize,

    /// The number of services (colors) to generate.
    #[serde(alias = "services")]
    pub number_of_services: usize,

    /// The service type definitions specifying the resource usage profiles.
    #[serde(alias = "service_types")]
    pub service_types: creo_lib::ServiceTypeVec,

    /// The (optional) seed for the RNG (defaults to a random, 16 characters long string).
    #[serde(default = "random_seed")]
    pub seed: String,

    /// The port the generated services start to publish on. The first service uses this port, while
    /// each subsequent service uses the next port number after the previous service's port. (Default: 30100)
    ///
    /// In other words, the generated application occupies the port range starting from the
    /// specified port `p` up to the ending port number `p+s-1`, where `s` is the number of
    /// services of the application.
    #[serde(default = "default_start_port")]
    pub start_port: u32,

    /// A list specifying inter-service calls of the application.
    ///
    /// This disables randomly drawing the inter-service call edges and allows to define the edges
    /// before hand.
    #[serde(alias = "service_call_list", default)]
    pub service_call_list: Vec<(usize, usize)>,
}

/// Returns the default port a generated application starts to publish on.
fn default_start_port() -> u32 {
    30100
}

/// Selects all programming languages as available during the generation.
fn select_all_programming_languages() -> Vec<ProgrammingLanguage> {
    ProgrammingLanguage::iter().collect()
}

/// Deserializes and validate a list of programming languages.
///
/// The list is invalid if it either contains duplicate languages or if it specifies selection
/// probabilities that do not sum up to `100`.
fn deserialize_languages<'de, D>(deserializer: D) -> Result<Vec<ProgrammingLanguage>, D::Error>
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

/// Returns a random, 16-character long seed.
fn random_seed() -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(16)
        .map(char::from)
        .collect()
}
