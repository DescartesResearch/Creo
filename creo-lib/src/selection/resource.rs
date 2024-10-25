use rand::{seq::SliceRandom, Rng};

use crate::service_types::{Resource, ServiceType};

pub fn select_resource<R: Rng>(service_type: &ServiceType, rng: &mut R) -> Resource {
    service_type
        .resources
        .choose_weighted(rng, |resource| resource.fraction)
        .expect("should be able to select a resource for a service type")
        .clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service_types;

    const ITER: usize = 10_000;

    #[test]
    fn test_resource_selection_single() {
        let cpu_label = service_types::Resource {
            resource: service_types::ResourceType::Cpu,
            fraction: 100,
            intensity: service_types::ResourceIntensity::High,
        };
        let memory_label = service_types::Resource {
            resource: service_types::ResourceType::Memory,
            fraction: 0,
            intensity: service_types::ResourceIntensity::High,
        };
        let service_type = service_types::ServiceType {
            fraction: 100,
            resources: Vec::from([cpu_label.clone(), memory_label]),
        };
        let mut rng = rand::thread_rng();
        for _ in 0..ITER {
            let selection = select_resource(&service_type, &mut rng);
            assert!(
                selection == cpu_label,
                "expected CPU label but found {}",
                selection
            );
        }
    }

    #[test]
    fn test_resource_selection_multi() {
        let cpu_label = service_types::Resource {
            resource: service_types::ResourceType::Cpu,
            fraction: 50,
            intensity: service_types::ResourceIntensity::High,
        };
        let memory_label = service_types::Resource {
            resource: service_types::ResourceType::Memory,
            fraction: 50,
            intensity: service_types::ResourceIntensity::High,
        };
        let service_type = service_types::ServiceType {
            fraction: 100,
            resources: Vec::from([cpu_label.clone(), memory_label.clone()]),
        };
        let mut cpu_count = 0;
        let mut memory_count = 0;
        let mut rng = rand::thread_rng();
        for _ in 0..ITER {
            let selection = select_resource(&service_type, &mut rng);
            if selection == cpu_label {
                cpu_count += 1;
                continue;
            }
            if selection == memory_label {
                memory_count += 1;
                continue;
            }
            panic!("unexpected service type: {}", selection);
        }
        // Maximum allowed difference: 10%
        let split = ITER / service_type.resources.len();
        let epsilon = split / 10;
        let min = split - epsilon;
        let max = split + epsilon;
        assert!(
            min <= cpu_count,
            "expected cpu label count to be at least {}, but was {}",
            min,
            cpu_count
        );
        assert!(
            max >= cpu_count,
            "expected cpu label count to be at most {}, but was {}",
            max,
            cpu_count
        );
        assert!(
            min <= memory_count,
            "expected memory label count to be at least {}, but was {}",
            min,
            memory_count
        );
        assert!(
            max >= memory_count,
            "expected memory label count to be at most {}, but was {}",
            max,
            memory_count
        );
    }
}
