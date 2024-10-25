use rand::{seq::SliceRandom, Rng};

use crate::service_types::ServiceType;

pub fn select_service_type<R: Rng>(
    available_service_types: &[ServiceType],
    rng: &mut R,
) -> ServiceType {
    available_service_types
        .choose_weighted(rng, |service_type| service_type.fraction)
        .expect("should be able to choose a random service type")
        .clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service_types;

    const ITER: usize = 10_000;

    #[test]
    fn test_service_type_selection_single() {
        let cpu_label = service_types::Resource {
            resource: service_types::ResourceType::Cpu,
            fraction: 100,
            intensity: service_types::ResourceIntensity::High,
        };
        let memory_label = service_types::Resource {
            resource: service_types::ResourceType::Memory,
            fraction: 100,
            intensity: service_types::ResourceIntensity::High,
        };
        let cpu_service_type = service_types::ServiceType {
            fraction: 100,
            resources: Vec::from([cpu_label]),
        };
        let service_types = [
            cpu_service_type.clone(),
            service_types::ServiceType {
                fraction: 0,
                resources: Vec::from([memory_label]),
            },
        ];
        let mut rng = rand::thread_rng();
        for _ in 0..ITER {
            let selection = select_service_type(&service_types, &mut rng);
            assert!(
                selection == cpu_service_type,
                "expected CPU service type but found {}",
                selection
            );
        }
    }

    #[test]
    fn test_service_type_selection_multi() {
        let cpu_label = service_types::Resource {
            resource: service_types::ResourceType::Cpu,
            fraction: 100,
            intensity: service_types::ResourceIntensity::High,
        };
        let memory_label = service_types::Resource {
            resource: service_types::ResourceType::Memory,
            fraction: 100,
            intensity: service_types::ResourceIntensity::High,
        };
        let cpu_service_type = service_types::ServiceType {
            fraction: 50,
            resources: Vec::from([cpu_label]),
        };
        let memory_service_type = service_types::ServiceType {
            fraction: 50,
            resources: Vec::from([memory_label]),
        };
        let service_types = [cpu_service_type.clone(), memory_service_type.clone()];
        let mut cpu_count = 0;
        let mut memory_count = 0;
        let mut rng = rand::thread_rng();
        for _ in 0..ITER {
            let selection = select_service_type(&service_types, &mut rng);
            if selection == cpu_service_type {
                cpu_count += 1;
                continue;
            }
            if selection == memory_service_type {
                memory_count += 1;
                continue;
            }
            panic!("unexpected service type: {}", selection);
        }
        // Maximum allowed difference: 10%
        let split = ITER / service_types.len();
        let epsilon = split / 10;
        let min = split - epsilon;
        let max = split + epsilon;
        assert!(
            min <= cpu_count,
            "expected cpu count to be at least {}, but was {}",
            min,
            cpu_count
        );
        assert!(
            max >= cpu_count,
            "expected cpu count to be at most {}, but was {}",
            max,
            cpu_count
        );
        assert!(
            min <= memory_count,
            "expected memory count to be at least {}, but was {}",
            min,
            memory_count
        );
        assert!(
            max >= memory_count,
            "expected memory count to be at most {}, but was {}",
            max,
            memory_count
        );
    }
}
