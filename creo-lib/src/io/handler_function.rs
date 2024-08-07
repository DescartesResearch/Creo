use crate::de::{FromJsonReader, FromYamlReader};
use crate::handler;

use super::Error;

pub fn parse_handler_function(
    handler_dir: impl AsRef<std::path::Path>,
) -> std::io::Result<handler::Function> {
    let file = crate::io::detect_file_with_file_name(handler_dir, "definition")?;
    let ft = crate::io::get_supported_file_type(file.as_ref())?;
    match ft {
        crate::io::FileType::YAML => Ok(handler::Function::from_yaml_reader(std::fs::File::open(
            file,
        )?)
        .map_err(Error::from)?),
        crate::io::FileType::JSON => Ok(handler::Function::from_json_reader(std::fs::File::open(
            file,
        )?)
        .map_err(Error::from)?),
    }
}
