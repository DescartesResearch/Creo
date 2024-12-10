pub fn validate_service_assignment(
    assignments: impl AsRef<[crate::config::service::EndpointAssignment]>,
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
    if missing.len() > 0 {
        return Err(super::Error::IncompleteEndpointAssignment { missing });
    }

    Ok(())
}

pub fn validate_edge_count(vertices: usize, edges: usize, colors: usize) -> super::Result<()> {
    let maximum = vertices * colors;
    if edges > maximum {
        return Err(super::Error::InvalidEdgeCount {
            got: edges,
            vertices,
            colors,
            expected: maximum,
        });
    }
    Ok(())
}

pub fn validate_color_count(vertices: usize, colors: usize) -> super::Result<()> {
    if vertices > colors {
        return Err(super::Error::InvalidColorCount {
            got: colors,
            expected: vertices,
        });
    }

    Ok(())
}
