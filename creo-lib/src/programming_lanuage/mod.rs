mod data_type;
mod dependency;
mod docker;
mod file;
mod random;
mod symbol;

use std::str::FromStr;

#[derive(strum_macros::EnumIter, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum ProgrammingLanguage {
    Python(usize),
    Rust(usize),
}

use ProgrammingLanguage::*;

impl ProgrammingLanguage {
    /// This function should return the directory name of the respective programming language in
    /// the `handlers` directory.
    pub fn as_dir_name(&self) -> &'static str {
        match self {
            Python(_) => "python",
            Rust(_) => "rust",
        }
    }
    pub fn as_fraction(&self) -> usize {
        match self {
            Python(f) => *f,
            Rust(f) => *f,
        }
    }
}

impl std::fmt::Display for ProgrammingLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Python(_) => f.write_str("Python"),
            Rust(_) => f.write_str("Rust"),
        }
    }
}

impl FromStr for ProgrammingLanguage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(':') {
            Some(("python", fraction)) => {
                return Ok(Python(
                    fraction.parse::<usize>().map_err(|err| err.to_string())?,
                ))
            }
            Some(("rust", fraction)) => {
                return Ok(Rust(
                    fraction.parse::<usize>().map_err(|err| err.to_string())?,
                ))
            }
            _ => (),
        }
        match s {
            "python" => Ok(Python(1)),
            "rust" => Ok(Rust(1)),
            _ => Err(format!("unknown programming language {}", s)),
        }
    }
}

impl<'de> serde::Deserialize<'de> for ProgrammingLanguage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ProgrammingLanguage::from_str(&s).map_err(serde::de::Error::custom)
    }
}
