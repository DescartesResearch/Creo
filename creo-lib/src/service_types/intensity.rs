use std::fmt::Display;

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Bucket {
    #[serde(alias = "low", alias = "LOW")]
    Low,
    #[serde(alias = "medium", alias = "MEDIUM")]
    Medium,
    #[serde(alias = "high", alias = "HIGH")]
    High,
}

impl Display for Bucket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Bucket::Low => "LOW",
            Bucket::Medium => "MEDIUM",
            Bucket::High => "HIGH",
        };
        f.write_str(s)
    }
}
