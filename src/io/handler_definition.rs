use std::collections::HashMap;

use creo_lib::{handler, programming_language::ProgrammingLanguage};

use crate::{Error, Result};

pub fn parse_handler_definitions<'a>(
    root: impl AsRef<std::path::Path>,
    languages: impl Iterator<Item = &'a ProgrammingLanguage>,
) -> Result<HashMap<ProgrammingLanguage, Vec<handler::Definition>>> {
    let root = root.as_ref();
    let defs = creo_lib::io::parse_handler_definitions(root, languages).map_err(|err| {
        Error::new(format!(
            "failed to obtain handler functions from path {}!\n\tReason: {}",
            root.display(),
            err
        ))
    })?;

    for (key, def) in &defs {
        if def.len() < 3 {
            return Err(Error::new(format!(
                "at least 3 handler functions are required for language {}",
                key
            )));
        }
    }

    Ok(defs)
}

pub fn glob_language_handler_definitions(
    root: impl AsRef<std::path::Path>,
    language: &ProgrammingLanguage,
) -> Result<Vec<std::path::PathBuf>> {
    creo_lib::io::glob_language_handler_definitions(root.as_ref(), language).map_err(|err| {
        Error::new(format!(
            "failed to obtain handler functions from path {} for language {}!\n\tReason: {}",
            root.as_ref().display(),
            language,
            err
        ))
    })
}
