const MEGABYTE_IN_BYTES: usize = 1024 * 1000;
pub async fn upload_and_extract_archive(
    client: &crate::ssh::Client,
    dst: impl AsRef<str>,
    src: impl AsRef<std::path::Path>,
) -> super::Result<()> {
    let dst = dst.as_ref();
    log::debug!("Creating remote archive file at path `{dst}`");
    let remote_file = client.create(dst).await?;
    log::debug!("Successfully created remote archive file at path `{dst}`");

    let src_file = tokio::fs::File::open(src.as_ref()).await?;
    let file_size = src_file.metadata().await?.len();
    let progress = indicatif::ProgressBar::new(file_size);
    progress.set_prefix(format!(
        "Uploading {}",
        src.as_ref().file_name().unwrap().to_string_lossy()
    ));
    let style = indicatif::ProgressStyle::with_template(
        "[{prefix}]: {wide_bar} {decimal_bytes_per_sec} {decimal_bytes}/{decimal_total_bytes}",
    )
    .unwrap()
    .progress_chars("##-");
    progress.set_style(style);

    let w = &mut progress.wrap_async_write(remote_file);

    tokio::io::copy(
        &mut tokio::io::BufReader::with_capacity(MEGABYTE_IN_BYTES * 16, src_file),
        w,
    )
    .await?;

    client
        .execute(format!(r#"tar -xzf "{}" && rm "{}""#, dst, dst))
        .await?;

    Ok(())
}
