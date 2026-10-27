//! Read-only scan entry point.

use std::{io, path::Path};

use crate::ScanReport;

/// Read-only file count and total size for a path.
pub fn scan(_root: &Path) -> io::Result<ScanReport> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "scan is not implemented",
    ))
}
