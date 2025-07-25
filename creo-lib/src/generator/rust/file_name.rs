use crate::generator::core::{self, FileName};

pub struct FileNameGenerator;

impl core::FileNameGenerator for FileNameGenerator {
    fn generate_router_file_name(&self) -> FileName {
        FileName {
            path: "src/router",
            extension: "rs",
        }
    }

    fn generate_service_call_file_name(&self) -> FileName {
        FileName {
            path: "src/router/service_calls",
            extension: "rs",
        }
    }

    fn generate_main_file_name(&self) -> FileName {
        FileName {
            path: "src/main",
            extension: "rs",
        }
    }
}
