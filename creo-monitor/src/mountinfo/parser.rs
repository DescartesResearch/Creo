//! Mountinfo line parser for Linux systems.
//!
//! Parses lines in `/proc/[pid]/mountinfo` format. See
//! [`proc_pid_mountinfo(5)`](https://man7.org/linux/man-pages/man5/proc_pid_mountinfo.5.html)
//! for details on the structure.

/// Represents a parsed mountinfo line.
#[derive(Debug, PartialEq, Eq)]
pub struct MountInfo<'a> {
    /// Mount ID field.
    pub mount_id: &'a str,
    /// Parent mount ID field.
    pub parent_id: &'a str,
    /// Major:Minor device identifier.
    pub major_minor: &'a str,
    /// Root of the mount within the filesystem.
    pub root: &'a str,
    /// Mount point relative to the process's root.
    pub mount_point: &'a str,
    /// Optional fields (can be empty).
    pub optional_fields: Vec<&'a str>,
    /// Filesystem type (e.g., `ext4`, `cgroup2`).
    pub fs_type: &'a str,
    /// Source of the mount (e.g., device).
    pub source: &'a str,
    /// Superblock options.
    pub super_options: &'a str,
}

/// Named fields in a mountinfo line.
#[derive(Debug)]
pub enum MountInfoField {
    MountId,
    ParentId,
    MajorMinor,
    Root,
    MountPoint,
    FsType,
    Source,
    SuperOptions,
}

impl std::fmt::Display for MountInfoField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            MountInfoField::MountId => "mount_id",
            MountInfoField::ParentId => "parent_id",
            MountInfoField::MajorMinor => "major:minor",
            MountInfoField::Root => "root",
            MountInfoField::MountPoint => "mount_point",
            MountInfoField::FsType => "fs_type",
            MountInfoField::Source => "source",
            MountInfoField::SuperOptions => "super_options",
        };
        write!(f, "{name}")
    }
}

/// Errors that may occur when parsing a mountinfo line.
#[derive(Debug, thiserror::Error)]
#[allow(clippy::enum_variant_names)]
pub enum ParseError {
    #[error("missing separator ` - ` in line: `{0}`")]
    MissingSeparator(String),

    #[error("missing `{field}` in pre-separator section of line: `{line}`")]
    MissingPreSeparatorField { field: MountInfoField, line: String },

    #[error("missing `{field}` in post-separator section of line: `{line}`")]
    MissingPostSeparatorField { field: MountInfoField, line: String },
}

/// Parses a single line of mountinfo data.
///
/// The line must follow the Linux kernel format described in [`proc_pid_mountinfo(5)`](https://man7.org/linux/man-pages/man5/proc_pid_mountinfo.5.html).
/// This function performs zero-allocation parsing except for collecting optional fields.
///
/// # Arguments
///
/// * `line` - A single line from `/proc/[pid]/mountinfo`.
///
/// # Returns
///
/// On success, returns a [`MountInfo`] struct referencing fields in the original input line.
///
/// # Errors
///
/// Returns [`ParseError`] variants for missing separator or required fields.
pub fn parse_mount_info_line<'a>(line: &'a str) -> Result<MountInfo<'a>, ParseError> {
    let (pre, post) = line
        .split_once(" - ")
        .ok_or_else(|| ParseError::MissingSeparator(line.to_owned()))?;

    let mut pre_fields = pre.split_whitespace();
    let mount_id = pre_fields
        .next()
        .ok_or_else(|| ParseError::MissingPreSeparatorField {
            field: MountInfoField::MountId,
            line: line.to_owned(),
        })?;
    let parent_id = pre_fields
        .next()
        .ok_or_else(|| ParseError::MissingPreSeparatorField {
            field: MountInfoField::ParentId,
            line: line.to_owned(),
        })?;
    let major_minor = pre_fields
        .next()
        .ok_or_else(|| ParseError::MissingPreSeparatorField {
            field: MountInfoField::MajorMinor,
            line: line.to_owned(),
        })?;
    let root = pre_fields
        .next()
        .ok_or_else(|| ParseError::MissingPreSeparatorField {
            field: MountInfoField::Root,
            line: line.to_owned(),
        })?;
    let mount_point = pre_fields
        .next()
        .ok_or_else(|| ParseError::MissingPreSeparatorField {
            field: MountInfoField::MountPoint,
            line: line.to_owned(),
        })?;

    let optional_fields: Vec<&str> = pre_fields.collect();

    let mut post_fields = post.split_whitespace();
    let fs_type = post_fields
        .next()
        .ok_or_else(|| ParseError::MissingPostSeparatorField {
            field: MountInfoField::FsType,
            line: line.to_owned(),
        })?;
    let source = post_fields
        .next()
        .ok_or_else(|| ParseError::MissingPostSeparatorField {
            field: MountInfoField::Source,
            line: line.to_owned(),
        })?;
    let super_options =
        post_fields
            .next()
            .ok_or_else(|| ParseError::MissingPostSeparatorField {
                field: MountInfoField::SuperOptions,
                line: line.to_owned(),
            })?;

    Ok(MountInfo {
        mount_id,
        parent_id,
        major_minor,
        root,
        mount_point,
        optional_fields,
        fs_type,
        source,
        super_options,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_mountinfo_line_with_optional_fields() {
        let line = "42 35 0:22 / /mnt rw,nosuid - ext4 /dev/sda1 rw,data=ordered";
        let result = parse_mount_info_line(line).unwrap();

        assert_eq!(result.mount_id, "42");
        assert_eq!(result.parent_id, "35");
        assert_eq!(result.major_minor, "0:22");
        assert_eq!(result.root, "/");
        assert_eq!(result.mount_point, "/mnt");
        assert_eq!(result.fs_type, "ext4");
        assert_eq!(result.source, "/dev/sda1");
        assert_eq!(result.super_options, "rw,data=ordered");
        assert_eq!(result.optional_fields, vec!["rw,nosuid"]);
    }

    #[test]
    fn error_on_missing_separator() {
        let line = "42 35 0:22 / /mnt rw,nosuid ext4 /dev/sda1 rw";
        let err = parse_mount_info_line(line).unwrap_err();
        matches!(err, ParseError::MissingSeparator(_));
    }

    #[test]
    fn error_on_missing_fields() {
        let line = "42 35 0:22 / - ext4 /dev/sda1 rw";
        let err = parse_mount_info_line(line).unwrap_err();
        match err {
            ParseError::MissingPreSeparatorField { field, .. } => {
                assert_eq!(field.to_string(), "mount_point");
            }
            _ => panic!("Expected MissingPreSeparatorField"),
        }
    }

    #[test]
    fn parses_valid_line_with_optional_fields() {
        let line = "42 35 0:22 / /mnt rw,nosuid,nodev - ext4 /dev/sda1 rw,data=ordered";
        let result = parse_mount_info_line(line).unwrap();
        assert_eq!(result.optional_fields, vec!["rw,nosuid,nodev"]);
    }

    #[test]
    fn parses_valid_line_with_no_optional_fields() {
        let line = "36 25 0:32 / /sys - sysfs sysfs rw";
        let result = parse_mount_info_line(line).unwrap();
        assert_eq!(result.optional_fields.len(), 0);
        assert_eq!(result.fs_type, "sysfs");
    }

    #[test]
    fn parses_valid_line_with_multiple_optional_fields() {
        let line = "70 56 0:45 / /var rw,nosuid,nodev,noexec,relatime shared:20 - ext4 /dev/sdb1 rw,errors=remount-ro";
        let result = parse_mount_info_line(line).unwrap();
        assert_eq!(
            result.optional_fields,
            vec!["rw,nosuid,nodev,noexec,relatime", "shared:20"]
        );
    }

    #[test]
    fn error_on_missing_mount_point() {
        let line = "42 35 0:22 / - ext4 /dev/sda1 rw";
        let err = parse_mount_info_line(line).unwrap_err();
        match err {
            ParseError::MissingPreSeparatorField { field, .. } => {
                assert_eq!(field.to_string(), "mount_point");
            }
            _ => panic!("Expected MissingPreSeparatorField"),
        }
    }

    #[test]
    fn error_on_missing_post_separator_fields() {
        let line = "42 35 0:22 / /mnt - ext4 /dev/sda1";
        let err = parse_mount_info_line(line).unwrap_err();
        match err {
            ParseError::MissingPostSeparatorField { field, .. } => {
                assert_eq!(field.to_string(), "super_options");
            }
            _ => panic!("Expected MissingPostSeparatorField"),
        }
    }

    #[test]
    fn error_on_empty_line() {
        let err = parse_mount_info_line("").unwrap_err();
        matches!(err, ParseError::MissingSeparator(_));
    }
}
