pub fn get_local_handler_dependencies(
    lib_dir: impl AsRef<std::path::Path>,
) -> std::io::Result<Vec<String>> {
    let mut deps = Vec::default();
    for entry in lib_dir.as_ref().read_dir()? {
        let entry = entry?;
        let ft = entry.file_type()?;
        if ft.is_dir() {
            deps.push(format!(
                "lib/{}",
                entry
                    .file_name()
                    .to_str()
                    .expect("directory name should be valid UTF-8")
            ))
        } else {
            log::debug!("Skipping entry {}", entry.path().display());
        }
    }

    Ok(deps)
}
