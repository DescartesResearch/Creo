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
    use statrs::distribution::ContinuousCDF;

    use super::*;
    use crate::service_types;

    const COUNT: usize = 100_000;
    const P_VALUE: f64 = 0.05;

    #[test]
    fn test_service_type_selection_single() {
        let cpu_label = service_types::Property {
            label: service_types::Label::Cpu,
            fraction: 100,
            bucket: service_types::Bucket::High,
        };
        let memory_label = service_types::Property {
            label: service_types::Label::Memory,
            fraction: 100,
            bucket: service_types::Bucket::High,
        };
        let cpu_service_type = service_types::ServiceType {
            fraction: 100,
            properties: Vec::from([cpu_label]),
        };
        let service_types = [
            cpu_service_type.clone(),
            service_types::ServiceType {
                fraction: 0,
                properties: Vec::from([memory_label]),
            },
        ];
        let mut rng = rand::thread_rng();
        for _ in 0..COUNT {
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
        let cpu_label = service_types::Property {
            label: service_types::Label::Cpu,
            fraction: 100,
            bucket: service_types::Bucket::High,
        };
        let memory_label = service_types::Property {
            label: service_types::Label::Memory,
            fraction: 100,
            bucket: service_types::Bucket::High,
        };
        let cpu_service_type = service_types::ServiceType {
            fraction: 80,
            properties: Vec::from([cpu_label]),
        };
        let memory_service_type = service_types::ServiceType {
            fraction: 20,
            properties: Vec::from([memory_label]),
        };
        let types = [cpu_service_type.clone(), memory_service_type.clone()];
        let mut observations = vec![0_isize; types.len()];
        let mut rng = rand::thread_rng();
        for _ in 0..COUNT {
            let selection = select_service_type(&types, &mut rng);
            let idx = types
                .iter()
                .position(|typ| selection == *typ)
                .expect("to find a type that matches the selection");
            observations[idx] += 1;
        }
        let expected: Vec<_> = types
            .iter()
            .map(|typ| (typ.fraction as f64 / 100.0) * COUNT as f64)
            .collect();
        let chi_squared: f64 = observations
            .iter()
            .zip(expected)
            .map(|(got, want)| (got.pow(2) as f64 / want) - COUNT as f64)
            .sum();
        let chi_dist = statrs::distribution::ChiSquared::new((types.len() - 1) as f64).unwrap();
        let p = 1.0 - P_VALUE;
        let cutoff = chi_dist.inverse_cdf(p);
        assert!(
            chi_squared <= cutoff,
            "expected {} <= {}",
            chi_squared,
            cutoff
        );
    }
}
