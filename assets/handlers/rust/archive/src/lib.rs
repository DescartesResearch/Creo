const PROJECT_ROOT: &str = env!("CARGO_MANIFEST_DIR");
const PREFIX: &str = "archive-";
const TAR_EXT: &str = ".tar";
const ZIP_EXT: &str = ".gz";
const LENGTH: usize = PREFIX.len() + TAR_EXT.len() + ZIP_EXT.len();

pub async fn archive_files(compress: bool) {
    let id = uuid::Uuid::new_v4().to_string();
    let mut file_name = String::with_capacity(LENGTH + id.len());
    file_name.push_str(PREFIX);
    file_name.push_str(&id);
    file_name.push_str(TAR_EXT);
    if compress {
        file_name.push_str(ZIP_EXT);
    }
    let path = std::path::PathBuf::from(PROJECT_ROOT).join("src/data");
    let file_path = path.join(format!("archives/{file_name}", file_name=&file_name));
    let file = tokio::fs::File::create(&file_path).await.unwrap();
    let sync_file = tokio_util::io::SyncIoBridge::new(file);
    if compress {
        let enc = flate2::write::GzEncoder::new(sync_file, flate2::Compression::default());
        let w = tar::Builder::new(enc);
        tokio::task::spawn_blocking(||{write_archive(w, id, path)}).await.unwrap();
    } else {
        let w = tar::Builder::new(sync_file);
        tokio::task::spawn_blocking(||{write_archive(w, id, path)}).await.unwrap();
    };
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(90)).await;
        tokio::fs::remove_file(file_path).await.unwrap();
    });
}

fn write_archive<W: std::io::Write>(mut w: tar::Builder<W>, id: String, path: std::path::PathBuf) {
    w.append_dir_all(format!("backup-{id}/logs"), path.join("raw/")).unwrap();
    w.finish().unwrap();
}

