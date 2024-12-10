use std::fmt::Write;

#[derive(serde::Deserialize, Debug)]
#[serde(remote = "Self")]
pub struct Config {
    pub key_file: Option<String>,
    pub master_hosts: Vec<String>,
    pub worker_hosts: Vec<String>,
    pub user_name: String,
    pub password_file: Option<String>,
}

impl<'de> serde::Deserialize<'de> for Config {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut this = Self::deserialize(deserializer)?;
        if this.master_hosts.is_empty() {
            return Err(serde::de::Error::custom(
                "expected non-empty list of master hosts",
            ));
        }

        if this.worker_hosts.is_empty() {
            return Err(serde::de::Error::custom(
                "expected non-empty list of worker hosts",
            ));
        }

        if this.worker_hosts.len() < this.master_hosts.len() {
            return Err(serde::de::Error::custom(
                "number of worker hosts must be greater than or equal to the number of master hosts",
            ));
        }

        this.master_hosts.sort();
        this.worker_hosts.sort();

        if let Some(key_file) = this.key_file {
            this.key_file = Some(try_shell_expand_string::<D>(&key_file)?);
        }
        if let Some(password_file) = this.password_file {
            this.password_file = Some(try_shell_expand_string::<D>(&password_file)?);
        }

        Ok(this)
    }
}

fn try_shell_expand_string<'de, D>(s: impl AsRef<str>) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    match shellexpand::full(s.as_ref()) {
        Ok(expanded) => Ok(expanded.to_string()),
        Err(err) => match err.cause {
            std::env::VarError::NotPresent => Err(serde::de::Error::custom(format!(
                "expected environment variable {}, but was not present",
                err.var_name
            ))),
            std::env::VarError::NotUnicode(_) => Err(serde::de::Error::custom(format!(
                "expected environment variable {} to only contain valid Unicode",
                err.var_name
            ))),
        },
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct RPSArray(Vec<usize>);

impl serde::Serialize for RPSArray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut out = String::new();
        for rps in &self.0 {
            write!(&mut out, "{} ", rps).unwrap();
        }
        serializer.serialize_str(out.trim_end())
    }
}

const fn default_director_threads() -> usize {
    256
}

const fn default_timeout() -> usize {
    3000
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(remote = "Self")]
pub struct BenchmarkConfig {
    pub rps: RPSArray,
    pub iterations: usize,
    pub benchmark_duration: usize,
    #[serde(default = "default_director_threads")]
    pub director_threads: usize,
    pub virtual_users: usize,
    #[serde(default = "default_timeout")]
    pub timeout: usize,
    pub warmup_pause: usize,
    pub warmup_duration: usize,
    pub warmup_rps: usize,
    pub records: usize,
}

impl<'de> serde::Deserialize<'de> for BenchmarkConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut this = Self::deserialize(deserializer)?;

        if this.rps.0.is_empty() {
            return Err(serde::de::Error::custom("expected at least one RPS value"));
        }
        if this.iterations < 1 {
            return Err(serde::de::Error::custom(
                "expected iteration count to be greater or equal to 1",
            ));
        }

        this.rps.0.sort();

        Ok(this)
    }
}

impl serde::Serialize for BenchmarkConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        Self::serialize(self, serializer)
    }
}
