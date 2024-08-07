use rand::{seq::SliceRandom, Rng};

use crate::service_types::{Resource, ServiceType};

pub fn select_resource<R: Rng>(service_type: &ServiceType, rng: &mut R) -> Resource {
    service_type
        .resources
        .choose_weighted(rng, |resource| resource.fraction)
        .expect("should be able to select a resource for a service type")
        .clone()
}
