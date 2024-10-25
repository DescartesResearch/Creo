use std::fmt::Display;

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ResourceIntensity {
    #[serde(alias = "low", alias = "LOW")]
    Low,
    #[serde(alias = "medium", alias = "MEDIUM")]
    Medium,
    #[serde(alias = "high", alias = "HIGH")]
    High,
}

impl Display for ResourceIntensity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ResourceIntensity::Low => "LOW",
            ResourceIntensity::Medium => "MEDIUM",
            ResourceIntensity::High => "HIGH",
        };
        f.write_str(s)
    }
}
