fn main() -> std::io::Result<()> {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile_protos(
            &[
                "vendor/containerd/api/services/containers/v1/containers.proto",
                "vendor/containerd/api/services/events/v1/events.proto",
                "vendor/containerd/api/services/tasks/v1/tasks.proto",
                "vendor/containerd/api/services/namespaces/v1/namespace.proto",
                "vendor/containerd/api/events/container.proto",
                "vendor/containerd/api/events/task.proto",
                "vendor/containerd/api/types/runc/options/oci.proto",
            ],
            &["vendor/containerd"],
        )?;

    Ok(())
}
