// TODO: check if anything different from /rootfs/sys/fs/cgroup and /sys/fs/cgroup
// TODO: check if I can use /rootfs/var/run/containerd/containerd.sock

/// Entry point for the Creo Monitor container monitoring tool.
///
/// This binary initializes the monitoring system, connecting to a container runtime
/// (e.g., containerd), collecting resource usage via cgroups, and persisting data
/// to a MySQL database. It also starts an API server for querying metrics.
///
/// # Errors
///
/// Returns an error if initialization fails (e.g., missing environment variables,
/// database connection issues, or container runtime errors).
///
/// # Examples
///
/// ```bash
/// DATABASE_URL=mysql://user:pass@localhost/creo_monitor cargo run
/// ```
#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    creo_monitor::run().await
}
