pub mod application;
pub mod compose;
pub(crate) mod constants;
pub mod de;
mod dependencies;
pub mod generator;
pub mod graph;
pub mod handler;
mod http_method;
pub mod io;
mod load;
pub mod metrics;
pub mod programming_lanuage;
pub mod remote;
pub mod schema;
pub mod selection;
mod service_types;
pub mod ssh;
pub mod stats;
pub mod template;

pub use service_types::ServiceTypeVec;

pub const VERSION: &str = "1.0.0";
pub use constants::*;
