/// A struct representing a single module/symbol import
#[derive(serde::Serialize, Debug, PartialEq, Eq, Hash)]
pub struct Import {
    /// The import string.
    pub import: String,
}

impl Import {
    pub fn new(import: String) -> Self {
        Self { import }
    }
}
