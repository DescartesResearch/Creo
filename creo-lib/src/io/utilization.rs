use crate::de::{FromJsonReader, FromYamlReader};
use crate::service_types;

use super::Error;

pub fn parse_utilization_file(
    handler_dir: impl AsRef<std::path::Path>,
) -> std::io::Result<service_types::Utilization> {
    let file = crate::io::detect_file_with_file_name(handler_dir.as_ref(), "utilization")?;
    let ft = crate::io::get_supported_file_type(file.as_ref())?;
    match ft {
        crate::io::FileType::YAML => Ok(service_types::Utilization::from_yaml_reader(
            std::fs::File::open(file.as_ref())?,
        )
        .map_err(Error::from)?),
        crate::io::FileType::JSON => Ok(service_types::Utilization::from_json_reader(
            std::fs::File::open(file.as_ref())?,
        )
        .map_err(Error::from)?),
    }
}
