use tokio::io::AsyncWriteExt;

pub struct Builder<W: tokio::io::AsyncWrite + Unpin + Send> {
    w: tokio_tar::Builder<async_compression::tokio::write::GzipEncoder<W>>,
}

impl<W: tokio::io::AsyncWrite + Unpin + Send + 'static> Builder<W> {
    pub fn new_compressed(w: W) -> Self {
        let w = async_compression::tokio::write::GzipEncoder::new(w);
        let w = tokio_tar::Builder::new(w);
        Self { w }
    }
}

impl<W: tokio::io::AsyncWrite + Unpin + Send> Builder<W> {
    /// Adds a file to this archive with the given path as the name of the file in the archive.
    pub async fn append_file(
        &mut self,
        path: impl AsRef<std::path::Path>,
        file: &mut tokio::fs::File,
    ) -> super::Result<()> {
        self.w.append_file(path, file).await?;
        Ok(())
    }

    /// Adds a new entry to this archive with the specified path.
    pub async fn append_data_as_file<P: AsRef<std::path::Path>, R: tokio::io::AsyncRead + Unpin>(
        &mut self,
        path: P,
        data: R,
        size: u64,
    ) -> super::Result<()> {
        let mut header = tokio_tar::Header::new_gnu();
        header.set_size(size);
        header.set_cksum();

        self.w.append_data(&mut header, path, data).await?;
        Ok(())
    }

    /// Adds a directory and all of its contents (recursively) to this archive with the given path
    /// as the name of the directory in the archive.
    ///
    /// Optionally, provide a slice of exclude patterns to exclude any directory or file matching
    /// at least one of the patterns.
    ///
    /// Note, that only regular files can be added to the archive when using exclude patterns.
    pub async fn append_dir_all(
        &mut self,
        path: impl AsRef<std::path::Path>,
        src: impl AsRef<std::path::Path>,
        exclude: &[&str],
    ) -> super::Result<()> {
        if exclude.is_empty() {
            return self
                .w
                .append_dir_all(path, src.as_ref())
                .await
                .map_err(|err| super::Error::AppendArchive {
                    source_path: src.as_ref().into(),
                    source: err,
                });
        }
        let patterns: super::Result<Vec<_>> = exclude
            .iter()
            .map(|pattern| ExcludePattern::new(pattern))
            .collect();
        let patterns = patterns?;
        self.append_dir_all_with_exclude(path, src.as_ref(), &patterns)
            .await
            .map_err(|err| super::Error::AppendArchive {
                source_path: src.as_ref().into(),
                source: err,
            })
    }

    /// Adds a directory and all of its contents not matched by [`Builder::should_exclude`] (recursively) to this archive with the given path
    /// as the name of the directory in the archive.
    ///
    /// This function only supports adding regular files to the archive.
    /// Internally, this function adds all included files to the archive by calling [`append_file`]
    /// on the builder.
    async fn append_dir_all_with_exclude(
        &mut self,
        path: impl AsRef<std::path::Path>,
        src_path: impl AsRef<std::path::Path>,
        exclude: &[ExcludePattern],
    ) -> std::io::Result<()> {
        let path = path.as_ref();
        let src_path = src_path.as_ref();
        let mut stack = vec![(src_path.to_path_buf(), true, false)];
        while let Some((src, is_dir, is_symlink)) = stack.pop() {
            if exclude
                .iter()
                .any(|pattern| self.should_exclude(&src, pattern))
            {
                continue;
            }
            let dest = path.join(src.strip_prefix(src_path).unwrap());
            // TODO: handle symlinks
            if is_dir {
                let mut entries = tokio::fs::read_dir(&src).await?;
                while let Some(entry) = entries.next_entry().await.transpose() {
                    let entry = entry?;
                    let ft = entry.file_type().await?;
                    stack.push((entry.path(), ft.is_dir(), ft.is_symlink()));
                }
                if dest != std::path::Path::new("") {
                    self.w.append_dir(&dest, &src).await?;
                }
            } else if is_symlink {
                let ft = src.metadata()?.file_type();
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("unsupported file type: {:?} for path {}", ft, src.display()),
                ))?;
            } else {
                self.w
                    .append_file(&dest, &mut tokio::fs::File::open(src).await?)
                    .await?
            }
        }
        Ok(())
    }

    fn should_exclude(&self, path: impl AsRef<std::path::Path>, pattern: &ExcludePattern) -> bool {
        let path = path.as_ref();
        if pattern.is_dir_pattern && !path.is_dir() {
            return false;
        }
        pattern.pattern.matches_path(path)
    }

    pub async fn into_inner(self) -> super::Result<W> {
        let mut gz = self.w.into_inner().await?;
        gz.shutdown().await?;
        Ok(gz.into_inner())
    }
}

struct ExcludePattern {
    pattern: glob::Pattern,
    is_dir_pattern: bool,
}

impl ExcludePattern {
    pub fn new(src: &str) -> super::Result<Self> {
        let is_dir_pattern = src.ends_with("/");
        let src = src.strip_suffix("/").unwrap_or(src);
        let pattern = glob::Pattern::new(src)?;
        Ok(Self {
            pattern,
            is_dir_pattern,
        })
    }
}

pub async fn extract_archive(
    src: impl AsRef<std::path::Path>,
    dst: impl AsRef<std::path::Path>,
) -> super::Result<()> {
    let mut archive = tokio_tar::Archive::new(async_compression::tokio::bufread::GzipDecoder::new(
        tokio::io::BufReader::new(tokio::fs::File::open(src).await?),
    ));
    archive.unpack(dst).await?;

    Ok(())
}
