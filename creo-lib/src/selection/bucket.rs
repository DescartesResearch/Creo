use crate::{
    handler,
    service_types::{Resource, ResourceIntensity},
};

const N_BUCKETS: usize = 3;

pub fn select_bucket<'a>(
    handler_definitions: &'a [handler::Definition],
    resource: &Resource,
) -> &'a [handler::Definition] {
    let bucket_boundaries = determine_bucket_boundaries(handler_definitions.len());
    let bucket_index = select_bucket_index(resource);
    let &(start, stop) = &bucket_boundaries[bucket_index].to_owned();
    &handler_definitions[start..stop]
}

fn determine_bucket_boundaries(length: usize) -> [(usize, usize); 3] {
    let (remainder_low, remainder_medium) = match length % N_BUCKETS {
        0 => (0, 0),
        1 => (0, 1),
        2 => (1, 1),
        _ => unreachable!(),
    };
    let a_third = length / 3;
    let low = a_third + remainder_low;
    let medium = low + a_third + remainder_medium;
    let high = length;
    [(0, low), (low, medium), (medium, high)]
}

fn select_bucket_index(resource: &Resource) -> usize {
    match resource.intensity {
        ResourceIntensity::Low => 0,
        ResourceIntensity::Medium => 1,
        ResourceIntensity::High => 2,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::service_types;

    use super::*;

    #[test]
    fn test_bucket_boundaries_divisible() {
        let boundaries = determine_bucket_boundaries(9);

        assert_eq!(
            boundaries,
            [(0, 3), (3, 6), (6, 9)],
            "unexpected boundaries: {:?}",
            boundaries
        );
    }

    #[test]
    fn test_bucket_boundaries_remainder_one() {
        let boundaries = determine_bucket_boundaries(10);

        assert_eq!(
            boundaries,
            [(0, 3), (3, 7), (7, 10)],
            "unexpected boundaries: {:?}",
            boundaries
        );
    }

    #[test]
    fn test_bucket_boundaries_remainder_two() {
        let boundaries = determine_bucket_boundaries(11);

        assert_eq!(
            boundaries,
            [(0, 4), (4, 8), (8, 11)],
            "unexpected boundaries: {:?}",
            boundaries
        );
    }

    #[test]
    fn test_bucket_low() {
        let mut definitions = Vec::with_capacity(9);
        for i in 0..9 {
            definitions.push(handler::Definition {
                directory: std::path::PathBuf::from(format!("test/path/{i}")),
                utilization: HashMap::default(),
            });
        }

        let resource = service_types::Resource {
            resource: service_types::ResourceType::Cpu,
            fraction: 100,
            intensity: service_types::ResourceIntensity::Low,
        };

        let bucket = select_bucket(&definitions, &resource);
        assert_eq!(
            bucket,
            &[
                handler::Definition {
                    directory: std::path::PathBuf::from("test/path/0".to_string()),
                    utilization: HashMap::default(),
                },
                handler::Definition {
                    directory: std::path::PathBuf::from("test/path/1".to_string()),
                    utilization: HashMap::default(),
                },
                handler::Definition {
                    directory: std::path::PathBuf::from("test/path/2".to_string()),
                    utilization: HashMap::default(),
                }
            ],
            "unexpected low bucket: {:?}",
            bucket
        );
    }
    #[test]
    fn test_bucket_medium() {
        let mut definitions = Vec::with_capacity(9);
        for i in 0..9 {
            definitions.push(handler::Definition {
                directory: std::path::PathBuf::from(format!("test/path/{i}")),
                utilization: HashMap::default(),
            });
        }

        let resource = service_types::Resource {
            resource: service_types::ResourceType::Cpu,
            fraction: 100,
            intensity: service_types::ResourceIntensity::Medium,
        };

        let bucket = select_bucket(&definitions, &resource);
        assert_eq!(
            bucket,
            &[
                handler::Definition {
                    directory: std::path::PathBuf::from("test/path/3".to_string()),
                    utilization: HashMap::default(),
                },
                handler::Definition {
                    directory: std::path::PathBuf::from("test/path/4".to_string()),
                    utilization: HashMap::default(),
                },
                handler::Definition {
                    directory: std::path::PathBuf::from("test/path/5".to_string()),
                    utilization: HashMap::default(),
                }
            ],
            "unexpected medium bucket: {:?}",
            bucket
        );
    }
    #[test]
    fn test_bucket_high() {
        let mut definitions = Vec::with_capacity(9);
        for i in 0..9 {
            definitions.push(handler::Definition {
                directory: std::path::PathBuf::from(format!("test/path/{i}")),
                utilization: HashMap::default(),
            });
        }

        let resource = service_types::Resource {
            resource: service_types::ResourceType::Cpu,
            fraction: 100,
            intensity: service_types::ResourceIntensity::High,
        };

        let bucket = select_bucket(&definitions, &resource);
        assert_eq!(
            bucket,
            &[
                handler::Definition {
                    directory: std::path::PathBuf::from("test/path/6".to_string()),
                    utilization: HashMap::default(),
                },
                handler::Definition {
                    directory: std::path::PathBuf::from("test/path/7".to_string()),
                    utilization: HashMap::default(),
                },
                handler::Definition {
                    directory: std::path::PathBuf::from("test/path/8".to_string()),
                    utilization: HashMap::default(),
                }
            ],
            "unexpected high bucket: {:?}",
            bucket
        );
    }
}
