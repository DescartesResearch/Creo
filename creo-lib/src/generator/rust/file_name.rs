use crate::generator::core::{self, FileName};

pub struct FileNameGenerator;

impl core::FileNameGenerator for FileNameGenerator {
    fn generate_router_file_name(&self) -> FileName {
        FileName {
            name: "src/router",
            extension: "rs",
        }
    }

    fn generate_service_call_file_name(&self) -> FileName {
        FileName {
            name: "src/service_calls",
            extension: "rs",
        }
    }

    fn generate_main_file_name(&self) -> FileName {
        FileName {
            name: "src/main",
            extension: "rs",
        }
    }
}
