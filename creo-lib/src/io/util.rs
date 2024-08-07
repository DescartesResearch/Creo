use crate::io::FileType;

pub fn is_empty_dir(path: impl AsRef<std::path::Path>) -> bool {
    path.as_ref()
        .read_dir()
        .map(|mut i| i.next().is_none())
        .unwrap_or(false)
}

pub fn create_dir_all(path: impl AsRef<std::path::Path>) -> std::io::Result<()> {
    match std::fs::create_dir_all(path.as_ref()) {
        Ok(_) => Ok(()),
        Err(err) => match err.kind() {
            std::io::ErrorKind::AlreadyExists => {
                if !is_empty_dir(path.as_ref()) {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::AlreadyExists,
                        format!(
                            "directory at path {} already exists and is not empty",
                            path.as_ref().display()
                        ),
                    ));
                }
                Ok(())
            }
            std::io::ErrorKind::PermissionDenied => {
                Err(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    format!(
                        "permission denied to create directory at path: {}",
                        path.as_ref().display()
                    ),
                ))
            }
            _ => Err(err),
        },
    }
}

pub fn is_dot_file(file: impl AsRef<std::path::Path>) -> bool {
    match file.as_ref().file_stem() {
        Some(fs) => fs.to_string_lossy().starts_with("."),
        None => false,
    }
}

pub fn is_dot_dir(dir: impl AsRef<std::path::Path>) -> bool {
    match dir.as_ref().file_name() {
        Some(dn) => dn.to_string_lossy().starts_with("."),
        None => false,
    }
}

pub fn copy_dir_all(
    src: impl AsRef<std::path::Path>,
    dst: impl AsRef<std::path::Path>,
) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let path = entry.path();
        if is_dot_dir(&path) {
            log::debug!("Skipping dot dir at path {}", path.display());
            continue;
        }
        if ty.is_dir() {
            copy_dir_all(&path, dst.as_ref().join(entry.file_name()))?;
        } else {
            if is_dot_file(&path) {
                log::debug!("Skipping dot file at path {}", path.display());
                continue;
            }
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}

pub fn get_supported_file_type<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<FileType> {
    let ext = path.as_ref().extension().ok_or(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        format!(
        "expected file name to include a `.yaml`, `.yml`, or `.json` extension, but the file name was {:?}",
        path.as_ref().file_name())
    ))?;
    match ext {
        _ if ext == "yml" || ext == "yaml" => Ok(FileType::YAML),
        _ if ext == "json" => Ok(FileType::JSON),
        _ => {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(
                    "expected file extension to be `.yaml`, `.yml`, or `.json`, but was .{:?}",
                    ext
                ),
            ))
        }
    }
}

pub fn detect_file_with_file_name(
    path: impl AsRef<std::path::Path>,
    file_name: impl AsRef<std::path::Path>,
) -> std::io::Result<impl AsRef<std::path::Path>> {
    let mut file = path.as_ref().join(&file_name);

    for ext in ["yml", "yaml", "json"] {
        file.set_extension(ext);
        if file.is_file() {
            return Ok(file);
        }
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        format!(
            "expected a json or yaml file named {} in path {}",
            file_name.as_ref().display(),
            path.as_ref().display()
        ),
    ))
}

pub async fn list_service_directories(
    path: impl AsRef<std::path::Path>,
) -> std::io::Result<Vec<std::path::PathBuf>> {
    let mut sub_dirs = Vec::default();
    let mut dir = tokio::fs::read_dir(&path).await?;
    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();
        if !path.is_dir() {
            log::debug!(
                "Skipping path `{}`, since it is not a directory",
                path.display()
            );
            continue;
        }
        if !path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .starts_with("handler")
        {
            log::warn!("Skipping unexpected directory at path {}!", path.display());
            continue;
        }
        sub_dirs.push(path);
    }
    Ok(sub_dirs)
}
