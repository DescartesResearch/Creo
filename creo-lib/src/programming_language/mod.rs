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
    /// Returns the directory name of the respective programming language in
    /// the `assets/handlers` directory.
    pub fn as_dir_name(&self) -> &'static str {
        match self {
            Python(_) => "python",
            Rust(_) => "rust",
        }
    }

    /// Returns the fraction weight value of the programming language.
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

    /// Parses the given string to a programming language. If the input contains a `:`, the part
    /// before the `:` should be treated as the programming language name, while the part after the
    /// `:` should be the fractional weight value. Otherwise, the entire input represents the
    /// programming language name.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, fraction) = s.split_once(':').unwrap_or((s, "1"));

        let fraction = fraction.parse::<usize>().map_err(|e| e.to_string())?;

        match name {
            "python" => Ok(Python(fraction)),
            "rust" => Ok(Rust(fraction)),
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
