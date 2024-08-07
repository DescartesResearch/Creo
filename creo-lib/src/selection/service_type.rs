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
