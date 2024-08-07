use std::collections::HashMap;

use creo_lib::{handler, programming_lanuage::ProgrammingLanguage};

use crate::{Error, Result};

pub fn parse_handler_definitions<'a>(
    handler_root_dir: impl AsRef<std::path::Path>,
    languages: impl Iterator<Item = &'a ProgrammingLanguage>,
) -> Result<HashMap<ProgrammingLanguage, Vec<handler::Definition>>> {
    let defs =
        creo_lib::io::parse_handler_definitions(&handler_root_dir, languages).map_err(|err| {
            Error::new(format!(
                "failed to obtain handler functions from path {}!\n\tReason: {}",
                handler_root_dir.as_ref().display(),
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
    root_handler_dir: impl AsRef<std::path::Path>,
    language: &ProgrammingLanguage,
) -> Result<Vec<std::path::PathBuf>> {
    creo_lib::io::glob_language_handler_definitions(root_handler_dir.as_ref(), language).map_err(|err| {
        Error::new(format!("failed to obtain handler functions from path {} for language {}!\n\tReason: {}", root_handler_dir.as_ref().display(), language, err))
    })
}
