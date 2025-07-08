use std::collections::HashMap;

use hyper_util::rt::TokioIo;
use tonic::Request;
use tonic::metadata::MetadataValue;
use tonic::transport::{Channel, Endpoint};
use tower::service_fn;

use crate::containerd::services::containers::v1::containers_client::ContainersClient;
use crate::containerd::services::containers::v1::{
    Container, GetContainerRequest, ListContainersRequest,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ContainerMeta {
    name: Option<String>,
}

impl ContainerMeta {
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PodMeta {
    name: Option<String>,
    namespace: Option<String>,
}

impl PodMeta {
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn namespace(&self) -> Option<&str> {
        self.namespace.as_deref()
    }
}

use super::{ContainerID, Error, Result};

#[derive(Debug, Clone)]
pub struct ContainerDMetaDataProvider {
    client: Option<ContainersClient<Channel>>,
    cache: HashMap<String, (ContainerMeta, PodMeta)>,
}

impl ContainerDMetaDataProvider {
    pub async fn new() -> Self {
        let channel = Endpoint::from_static("http://[::]:50051")
            .connect_with_connector(service_fn(|_| async {
                Ok::<_, std::io::Error>(TokioIo::new(
                    tokio::net::UnixStream::connect("/var/run/containerd/containerd.sock").await?,
                ))
            }))
            .await
            .ok();

        let mut client = channel.map(ContainersClient::new);
        let mut cache = HashMap::default();

        if let Some(ref mut client) = client {
            let mut request = Request::new(ListContainersRequest { filters: vec![] });
            request
                .metadata_mut()
                .insert("containerd-namespace", MetadataValue::from_static("k8s.io"));

            match client.list_stream(request).await {
                Ok(r) => {
                    let mut stream = r.into_inner();
                    while let Some(container_msg) = stream.message().await.ok().flatten() {
                        if let Some(container) = container_msg.container {
                            let (id, container_meta, pod_meta) =
                                extract_containerd_metadata(container);
                            cache.insert(id, (container_meta, pod_meta));
                        };
                    }
                }
                Err(err) => {
                    log::error!("failed to request containers list: {err}")
                }
            };
        }

        Self { client, cache }
    }

    pub async fn request_metadata(
        &mut self,
        container_id: ContainerID,
    ) -> Result<Option<(ContainerMeta, PodMeta)>> {
        let id = container_id.to_string();
        log::debug!("Requesting metadata for ID: {id}");
        if let Some((container_meta, pod_meta)) = self.cache.remove(&id) {
            log::debug!(
                "Found container metadata in cache: id={id}, container_meta={container_meta:?}, pod_meta={pod_meta:?}"
            );
            return Ok(Some((container_meta, pod_meta)));
        }

        if let Some(ref mut client) = self.client {
            log::debug!("Requesting metadata from containerd socket for ID: {id}");
            let mut request = Request::new(GetContainerRequest { id });
            request
                .metadata_mut()
                .insert("containerd-namespace", MetadataValue::from_static("k8s.io"));
            let response = client
                .get(request)
                .await
                .map_err(|source| Error::ContainerDRequestError(Box::new(source)))?
                .into_inner();

            match response.container {
                Some(container) => {
                    log::debug!(
                        "Extracting container and pod metadata from response for ID: {}",
                        container.id
                    );
                    let (id, container_meta, pod_meta) = extract_containerd_metadata(container);
                    log::debug!(
                        "Extracted metadata: id={}, container_meta={container_meta:?}, pod_meta={pod_meta:?}",
                        id
                    );
                    return Ok(Some((container_meta, pod_meta)));
                }
                None => return Ok(None),
            }
        }
        Ok(None)
    }
}

fn extract_containerd_metadata(container: Container) -> (String, ContainerMeta, PodMeta) {
    let id = container.id;
    // Labels for container:
    // id=0048f4f5b68c91bdf14ddc7ef934d77512924ae6c7d037f61c89b83d04d7ceeb,
    // labels={
    //  "io.kubernetes.container.name": "istio-init",
    //  "org.opencontainers.image.ref.name": "ubuntu",
    //  "io.kubernetes.pod.uid": "9801db07-caaf-424c-8fcf-0f88ee45908b",
    //  "org.opencontainers.image.version": "24.04",
    //  "io.kubernetes.pod.namespace": "default",
    //  "io.kubernetes.pod.name": "nginx-deployment-d556bf558-px5xv",
    //  "io.cri-containerd.kind": "container"
    //  }
    log::debug!(
        "Labels for container: id={}, labels={:?}",
        &id,
        &container.labels
    );
    let container_name = container
        .labels
        .get("io.kubernetes.container.name")
        .map(|n| n.to_owned());
    let pod_name = container
        .labels
        .get("io.kubernetes.pod.name")
        .map(|n| n.to_owned());
    let pod_namespace = container
        .labels
        .get("io.kubernetes.pod.namespace")
        .map(|n| n.to_owned());

    (
        id,
        ContainerMeta {
            name: container_name,
        },
        PodMeta {
            name: pod_name,
            namespace: pod_namespace,
        },
    )
}
