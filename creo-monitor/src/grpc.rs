use std::path::{Path, PathBuf};
use std::{pin, task};

use hyper_util::rt::TokioIo;
use tonic::transport::{Channel, Endpoint};

#[derive(Debug, Clone)]
struct UnixConnector {
    path: PathBuf,
}

impl tower::Service<hyper::Uri> for UnixConnector {
    type Response = TokioIo<tokio::net::UnixStream>;

    type Error = std::io::Error;

    type Future = pin::Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: hyper::Uri) -> Self::Future {
        let path = self.path.clone();
        Box::pin(async move {
            let stream = tokio::net::UnixStream::connect(path).await?;

            Ok(TokioIo::new(stream))
        })
    }
}

pub async fn channel_for_unix_socket(
    path: impl AsRef<Path>,
) -> Result<Channel, tonic::transport::Error> {
    let path = path.as_ref();
    log::debug!("Connecting to {}...", path.display());
    let connector = UnixConnector {
        path: path.to_path_buf(),
    };
    log::debug!("Connected to {}. Creating channel...", path.display());
    let channel = Endpoint::from_static("http://[::]:50051")
        .connect_with_connector(connector)
        .await?;
    log::debug!("Created channel for {}.", path.display());

    Ok(channel)
}
