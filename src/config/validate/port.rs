/// The minimal valid port value.
const MINIMUM_PORT: u32 = 30000;
/// The maximal valid port value.
const MAXIMUM_PORT: u32 = 49151;

/// Validates that the given port represent a valid port number. The given
pub fn port(start: u32, end: u32) -> super::Result<()> {
    if start >= end || start < MINIMUM_PORT || end > MAXIMUM_PORT {
        return Err(super::Error::InvalidPortRange {
            start,
            end,
            min: MINIMUM_PORT,
            max: MAXIMUM_PORT,
        });
    }
    Ok(())
}
