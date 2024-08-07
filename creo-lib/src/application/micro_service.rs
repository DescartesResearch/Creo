use crate::{graph::MicroServiceIndex, programming_lanuage::ProgrammingLanguage};

pub struct MicroService {
    pub id: MicroServiceIndex,
    pub language: ProgrammingLanguage,
    pub port: u32,
}

impl MicroService {
    pub fn new(id: MicroServiceIndex, language: ProgrammingLanguage, port: u32) -> Self {
        Self { id, language, port }
    }

    pub fn as_dir_name(&self, digits: usize) -> String {
        format!("service-{:0>width$}", self.id.0, width = digits)
    }
}

pub fn get_host(id: MicroServiceIndex) -> String {
    format!("service-{}", id.0)
}
