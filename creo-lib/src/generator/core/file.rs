pub trait FileNameGenerator {
    /// The name of the router source code file.
    fn generate_router_file_name(&self) -> FileName;
    /// The name of the service call source code file.
    fn generate_service_call_file_name(&self) -> FileName;
    /// The name of the main source code file.
    fn generate_main_file_name(&self) -> FileName;
}

pub struct FileName {
    /// The name of the file.
    pub name: &'static str,
    /// The extension of the file **without** a leading '.', e.g., "py", "rs", or "go".
    pub extension: &'static str,
}

impl FileName {
    pub fn as_complete_file_name(&self) -> String {
        format!("{}.{}", self.name, self.extension)
    }
}
