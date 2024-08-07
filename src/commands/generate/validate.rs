use crate::cli;
pub use crate::{Error, Result};

pub(super) fn validate_arguments(args: &cli::generate::GenerateConfig) -> Result<()> {
    validate_number_of_service_calls(
        args.number_of_endpoints,
        args.number_of_service_calls,
        args.number_of_services,
    )?;
    validate_number_of_services(args.number_of_endpoints, args.number_of_services)?;
    validate_port_range(
        args.start_port,
        args.start_port + (args.number_of_services as u32),
    )?;

    Ok(())
}

/// Validates that the passed number of service calls and number of services allow for a proper
/// coloring of the generated graph.
fn validate_number_of_service_calls(
    endpoints: usize,
    service_calls: usize,
    services: usize,
) -> Result<()> {
    let maximum_valid_service_calls = endpoints * services;

    if service_calls > maximum_valid_service_calls {
        let msg = format!(
        "number of service calls must not exceed the product of the number of endpoints and the number of services. Expected at most {} services calls, but got {}",
        maximum_valid_service_calls,
        service_calls
    );
        return Err(Error::new(msg));
    }

    Ok(())
}

/// Validates that the number of services is less than or equal to the number of endpoints.
fn validate_number_of_services(endpoints: usize, services: usize) -> Result<()> {
    if services > endpoints {
        let msg = format!(
            "number of services must not exceed number of endpoints. Expected at most {} services, but got {}",
        endpoints,
        services
    );
        return Err(Error::new(msg));
    }

    Ok(())
}

/// Validates that the given number is a valid port
fn validate_port_range(start: u32, end: u32) -> Result<()> {
    if start >= end || start < 1024 || end > 49151 {
        let msg = format!("invalid port range. Port range should be in the range 1025..=49151 for applications, but was {}..={}",
        start,
        end
    );
        return Err(Error::new(msg));
    }

    Ok(())
}
