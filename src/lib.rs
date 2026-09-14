//! Read-only filesystem scanning, independent of CLI output.

mod filesystem;
mod model;
mod scan;

pub use model::ScanReport;
pub use scan::scan;
