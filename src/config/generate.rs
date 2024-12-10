use rand::Rng;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    /// The application name.
    #[serde(alias = "application_name", alias = "name")]
    pub app_name: creo_lib::de::NonEmptyString,

    /// The (optional) seed for the RNG (defaults to a random, 16 characters long string).
    #[serde(default = "random_seed")]
    pub seed: String,

    /// The port the generated services start to publish on. The first service uses this port, while
    /// each subsequent service uses the next port number after the previous service's port. (Default: 30100)
    ///
    /// In other words, the generated application occupies the port range starting from the
    /// specified port `p` up to the ending port number `p+s-1`, where `s` is the number of
    /// services in the application.
    #[serde(default)]
    pub start_port: creo_lib::Port,

    #[serde(flatten)]
    pub mode: Mode,
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum Mode {
    AutoPilot {
        topology: super::graph::AutoPilotConfig,
        workload: super::application::AutoPilotConfig,
    },
    Hybrid {
        topology: super::graph::HypridConfig,
        workload: super::application::AutoPilotConfig,
    },
    Manual {
        #[serde(flatten)]
        application: super::graph::ManualConfig,
    },
}

/// Returns a random, 16-character long, alphanumeric seed.
fn random_seed() -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(16)
        .map(char::from)
        .collect()
}
