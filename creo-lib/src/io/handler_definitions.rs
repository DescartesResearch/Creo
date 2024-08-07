use std::collections::HashMap;

use crate::{handler, programming_lanuage::ProgrammingLanguage};

pub fn parse_handler_definitions<'a>(
    root_handler_dir: impl AsRef<std::path::Path>,
    programming_languages: impl IntoIterator<Item = &'a ProgrammingLanguage>,
) -> std::io::Result<HashMap<ProgrammingLanguage, Vec<handler::Definition>>> {
    let mut all_defs = HashMap::default();
    for lang in programming_languages {
        let defs = parse_language_handler_definition(root_handler_dir.as_ref(), lang)?;
        all_defs.insert(*lang, defs);
    }

    Ok(all_defs)
}

pub fn parse_language_handler_definition(
    root_handler_dir: impl AsRef<std::path::Path>,
    lang: &ProgrammingLanguage,
) -> std::io::Result<Vec<handler::Definition>> {
    let dirs = glob_language_handler_definitions(root_handler_dir.as_ref(), lang)?;
    let mut defs = Vec::with_capacity(dirs.len());
    for path in dirs {
        match crate::io::parse_utilization_file(&path) {
            Ok(utilization) => {
                let def = handler::Definition::new(&path, utilization);
                defs.push(def);
            }
            Err(err) => {
                log::warn!(
                    "Could not parse utilization file for path `{}`\n\tReason: {}",
                    path.display(),
                    err
                )
            }
        }
    }
    Ok(defs)
}

pub fn glob_language_handler_definitions(
    root_handler_dir: impl AsRef<std::path::Path>,
    lang: &ProgrammingLanguage,
) -> std::io::Result<Vec<std::path::PathBuf>> {
    let lang_dir = root_handler_dir.as_ref().join(lang.as_dir_name());

    if !lang_dir.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!(
                "expected the folder {} to exists in {} for language {}",
                lang.as_dir_name(),
                root_handler_dir.as_ref().display(),
                lang,
            ),
        ));
    }

    let mut handler_dirs = Vec::default();

    for entry in lang_dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            log::debug!("Skipping path {}", path.display());
            continue;
        }

        let is_valid = crate::io::detect_file_with_file_name(&path, "definition").is_ok();
        if !is_valid {
            log::warn!(
                "Skipping directory at path {}, as it does not contain a definition file",
                path.display()
            );
            continue;
        }

        handler_dirs.push(path);
    }

    if handler_dirs.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("did not find any handler functions for language {}", lang),
        ));
    }

    Ok(handler_dirs)
}
