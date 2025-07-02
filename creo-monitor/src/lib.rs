pub mod api;
pub mod cgroup;
pub mod container;
pub mod discovery;
pub mod error;
pub mod grpc;
pub mod persistence;
pub mod stats;

pub const CGROUP_ROOT: &str = "/sys/fs/cgroup";
// TODO: change based on host vs. containerized
pub const ROOT: &str = "/rootfs";

pub mod containerd {
    pub mod runc {
        pub mod v1 {
            tonic::include_proto!("containerd.runc.v1");
        }
    }
    pub mod v1 {
        pub mod types {
            tonic::include_proto!("containerd.v1.types");
        }
    }
    pub mod types {
        tonic::include_proto!("containerd.types");
    }
    pub mod events {
        tonic::include_proto!("containerd.events");
    }
    pub mod services {
        pub mod containers {
            pub mod v1 {
                tonic::include_proto!("containerd.services.containers.v1");
            }
        }
        pub mod events {
            pub mod v1 {
                tonic::include_proto!("containerd.services.events.v1");
            }
        }
        pub mod tasks {
            pub mod v1 {
                tonic::include_proto!("containerd.services.tasks.v1");
            }
        }
    }
}
