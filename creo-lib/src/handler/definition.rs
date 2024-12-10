use std::{collections::HashMap, fmt::Display};

use crate::service_types::{Label, Utilization};

use super::Function;

#[derive(Clone, Debug)]
pub struct Definition {
    pub directory: std::path::PathBuf,
    pub utilization: HashMap<Label, f64>,
}

impl Definition {
    pub fn new<P: AsRef<std::path::Path>>(handler_dir: P, utilization: Utilization) -> Self {
        Self {
            directory: handler_dir.as_ref().to_path_buf(),
            utilization: utilization.0,
        }
    }

    pub fn compare_by_resource_type(
        &self,
        other: &Definition,
        resource: &Label,
    ) -> std::cmp::Ordering {
        let self_util = self
            .utilization
            .get(resource)
            .unwrap_or_else(|| assert_utilization(self, resource));
        let other_util = other
            .utilization
            .get(resource)
            .unwrap_or_else(|| assert_utilization(other, resource));
        self_util.partial_cmp(other_util).unwrap_or_else(|| {
            panic!(
                "non NAN value for utilization of resource {} for {} and {}",
                resource, self_util, other_util
            )
        })
    }
}

fn assert_utilization(def: &Definition, resource: &Label) -> ! {
    panic!(
        "expected a utilization for resource {} of definition at path {}",
        resource,
        def.directory.display()
    )
}

impl TryInto<Function> for Definition {
    type Error = std::io::Error;

    fn try_into(self) -> Result<Function, Self::Error> {
        crate::io::parse_handler_function(self.directory)
    }
}

impl TryInto<Function> for &Definition {
    type Error = std::io::Error;

    fn try_into(self) -> Result<Function, Self::Error> {
        crate::io::parse_handler_function(&self.directory)
    }
}

impl PartialEq for Definition {
    fn eq(&self, other: &Self) -> bool {
        self.directory == other.directory
    }
}

impl Eq for Definition {}

impl Display for Definition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Definition ({})", self.directory.display())
    }
}

#[cfg(test)]
mod tests {
    use crate::service_types;

    use super::*;

    #[test]
    fn test_compare_by_resource() {
        let one = Definition {
            directory: std::path::PathBuf::from("test/path/1"),
            utilization: HashMap::from_iter([(service_types::Label::Cpu, 0.5)]),
        };
        let two = Definition {
            directory: std::path::PathBuf::from("test/path/2"),
            utilization: HashMap::from_iter([(service_types::Label::Cpu, 1.5)]),
        };

        let cmp = one.compare_by_resource_type(&two, &service_types::Label::Cpu);
        assert_eq!(
            cmp,
            std::cmp::Ordering::Less,
            "expected definition {} to be less than definition {}",
            one,
            two
        )
    }
}
