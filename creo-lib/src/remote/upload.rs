pub async fn upload_and_extract_archive(
    client: &crate::ssh::Client,
    dst: impl AsRef<str>,
    src: impl AsRef<std::path::Path>,
) -> super::Result<()> {
    let dst = dst.as_ref();
    let mut remote_file = client.create(dst).await?;

    tokio::io::copy(
        &mut tokio::fs::File::open(src.as_ref()).await?,
        &mut remote_file,
    )
    .await?;

    client
        .execute(format!(r#"tar -xzf "{}" && rm "{}""#, dst, dst))
        .await?;

    Ok(())
}
