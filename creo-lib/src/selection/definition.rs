use rand::seq::SliceRandom;

use crate::{handler, service_types};

pub fn select_definition(
    definitons: &mut [handler::Definition],
    resource: &service_types::Property,
    rng: &mut impl rand::Rng,
) -> handler::Definition {
    sort_language_definitions_by_resource_type(definitons, &resource.label);
    let bucket = super::select_bucket(definitons, resource);
    bucket.choose(rng).expect("non empty bucket").clone()
}

fn sort_language_definitions_by_resource_type(
    definitions: &mut [handler::Definition],
    resource_type: &service_types::Label,
) {
    definitions.sort_by(|a, b| a.compare_by_resource_type(b, resource_type));
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    const ITER: usize = 10_000;

    #[test]
    fn test_language_definitions_sorting() {
        let one = handler::Definition {
            directory: std::path::PathBuf::from("test/path/1"),
            utilization: HashMap::from_iter([(service_types::Label::Cpu, 0.5)]),
        };
        let two = handler::Definition {
            directory: std::path::PathBuf::from("test/path/2"),
            utilization: HashMap::from_iter([(service_types::Label::Cpu, 1.5)]),
        };
        let three = handler::Definition {
            directory: std::path::PathBuf::from("test/path/3"),
            utilization: HashMap::from_iter([(service_types::Label::Cpu, 0.8)]),
        };
        let mut definitions = [one.clone(), two.clone(), three.clone()];
        sort_language_definitions_by_resource_type(&mut definitions, &service_types::Label::Cpu);
        assert_eq!(
            &definitions,
            &[one, three, two],
            "unexpected definition ordering: {:?}",
            &definitions
        );
    }

    #[test]
    fn test_definition_selection_single_bucket() {
        let one = handler::Definition {
            directory: std::path::PathBuf::from("test/path/1"),
            utilization: HashMap::from_iter([(service_types::Label::Cpu, 0.5)]),
        };
        let two = handler::Definition {
            directory: std::path::PathBuf::from("test/path/2"),
            utilization: HashMap::from_iter([(service_types::Label::Cpu, 1.5)]),
        };
        let three = handler::Definition {
            directory: std::path::PathBuf::from("test/path/3"),
            utilization: HashMap::from_iter([(service_types::Label::Cpu, 0.8)]),
        };
        let mut definitions = [one.clone(), two.clone(), three.clone()];
        let resource = service_types::Property {
            label: service_types::Label::Cpu,
            fraction: 100,
            bucket: service_types::Bucket::High,
        };
        let mut rng = rand::thread_rng();

        for _ in 0..ITER {
            let selection = select_definition(&mut definitions, &resource, &mut rng);
            assert_eq!(selection, two, "unexpected selection: {}", selection)
        }
    }

    #[test]
    fn test_definition_selection_multi_bucket() {
        let one = handler::Definition {
            directory: std::path::PathBuf::from("test/path/1"),
            utilization: HashMap::from_iter([(service_types::Label::Cpu, 0.5)]),
        };
        let two = handler::Definition {
            directory: std::path::PathBuf::from("test/path/2"),
            utilization: HashMap::from_iter([(service_types::Label::Cpu, 1.5)]),
        };
        let three = handler::Definition {
            directory: std::path::PathBuf::from("test/path/3"),
            utilization: HashMap::from_iter([(service_types::Label::Cpu, 0.8)]),
        };
        let four = handler::Definition {
            directory: std::path::PathBuf::from("test/path/3"),
            utilization: HashMap::from_iter([(service_types::Label::Cpu, 0.2)]),
        };
        let five = handler::Definition {
            directory: std::path::PathBuf::from("test/path/3"),
            utilization: HashMap::from_iter([(service_types::Label::Cpu, 3.8)]),
        };
        let six = handler::Definition {
            directory: std::path::PathBuf::from("test/path/3"),
            utilization: HashMap::from_iter([(service_types::Label::Cpu, 0.7)]),
        };
        let mut definitions = [
            one.clone(),
            two.clone(),
            three.clone(),
            four.clone(),
            five.clone(),
            six.clone(),
        ];
        let resource = service_types::Property {
            label: service_types::Label::Cpu,
            fraction: 100,
            bucket: service_types::Bucket::High,
        };
        let mut rng = rand::thread_rng();

        for _ in 0..ITER {
            let selection = select_definition(&mut definitions, &resource, &mut rng);
            assert!(
                (selection == two) || (selection == five),
                "unexpected selection: {}",
                selection
            )
        }
    }
}
