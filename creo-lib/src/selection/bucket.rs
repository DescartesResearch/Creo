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
