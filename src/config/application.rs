use strum::IntoEnumIterator;

#[derive(Debug, serde::Deserialize)]
pub struct AutoPilotConfig {
    /// The service type definitions specifying the resource usage profiles.
    pub service_types: creo_lib::ServiceTypeVec,
    /// The programming languages that are available during the generation.
    #[serde(
        deserialize_with = "deserialize_languages",
        default = "select_all_programming_languages"
    )]
    pub programming_languages:
        creo_lib::de::UniqueVec<creo_lib::programming_language::ProgrammingLanguage>,
}

/// Selects all programming languages as available during the generation.
fn select_all_programming_languages(
) -> creo_lib::de::UniqueVec<creo_lib::programming_language::ProgrammingLanguage> {
    creo_lib::programming_language::ProgrammingLanguage::iter()
        .collect::<Vec<_>>()
        .into()
}

/// Deserializes and validate a list of programming languages.
///
/// The list is invalid if it either contains duplicate languages or if it specifies selection
/// probabilities that do not sum up to `100`.
fn deserialize_languages<'de, D>(
    deserializer: D,
) -> Result<creo_lib::de::UniqueVec<creo_lib::programming_language::ProgrammingLanguage>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let this =
        creo_lib::de::UniqueVec::<creo_lib::programming_language::ProgrammingLanguage>::deserialize(
            deserializer,
        )?;
    let sum: usize = this.iter().map(|l| l.as_fraction()).sum();
    if !this.iter().all(|l| l.as_fraction() == 1) && sum != 100 {
        return Err(serde::de::Error::custom(format!(
            "expected programming language fractions to sum to 100, but was {}",
            sum
        )));
    }
    Ok(this)
}
