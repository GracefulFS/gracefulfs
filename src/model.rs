//! Scan result types.

use std::time::Duration;

#[derive(Debug)]
pub struct ScanReport {
    pub files: u64,
    /// Logical size counted per path, not physical disk usage.
    pub logical_bytes: u64,
    pub elapsed: Duration,
}
