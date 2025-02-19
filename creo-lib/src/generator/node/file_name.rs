use crate::generator::core::{self, FileName};

pub struct FileNameGenerator;

impl core::FileNameGenerator for FileNameGenerator {
    fn generate_router_file_name(&self) -> FileName {
        FileName {
            name: "src/router",
            extension: "js",
        }
    }

    fn generate_service_call_file_name(&self) -> FileName {
        FileName {
            name: "src/service-calls",
            extension: "js",
        }
    }

    fn generate_main_file_name(&self) -> FileName {
        FileName {
            name: "src/index",
            extension: "js",
        }
    }
}
