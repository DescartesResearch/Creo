#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ResourceIntensity {
    #[serde(alias = "low", alias = "LOW")]
    Low,
    #[serde(alias = "medium", alias = "MEDIUM")]
    Medium,
    #[serde(alias = "high", alias = "HIGH")]
    High,
}
