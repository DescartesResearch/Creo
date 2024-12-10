pub fn validate_function_assignment(
    assignments: impl AsRef<[crate::config::application::HandlerFunctionAssignment]>,
    vertices: usize,
) -> super::Result<()> {
    let assignments = assignments.as_ref();
    let mut assigned = vec![false; vertices];
    let mut invalid = Vec::with_capacity(assignments.len());
    for assignment in assignments {
        if assignment.endpoint >= vertices {
            invalid.push(assignment.endpoint);
            continue;
        }
        assigned[assignment.endpoint] = true;
    }
    if !invalid.is_empty() {
        return Err(super::Error::InvalidEndpointsInAssignment {
            got: invalid,
            vertices,
        });
    }
    let mut missing = Vec::with_capacity(vertices);
    for (idx, is_assigned) in assigned.into_iter().enumerate() {
        if !is_assigned {
            missing.push(idx);
        }
    }
    if !missing.is_empty() {
        return Err(super::Error::IncompleteHandlerFunctionAssignment { missing });
    }

    Ok(())
}
