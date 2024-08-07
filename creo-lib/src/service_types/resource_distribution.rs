#[derive(serde::Deserialize, Clone, Debug)]
#[serde(remote = "Self")]
pub struct ResourceDistribution{
    pub low: u8,
    pub mid: u8,
    pub high: u8,
}

impl<'de> serde::Deserialize<'de> for ResourceDistribution {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let this = Self::deserialize(deserializer)?;

        let sum = this.low + this.mid + this.high;
        if sum != 100 {
            return Err(serde::de::Error::custom(format!(
                "expected fractions of resource distribution to sum up to 100%, but sum was {}% (= {}% + {}% + {}%)",
                sum, this.low, this.mid, this.high
            )));
        }

        Ok(this)
    }
}
