//! Integration tests for the public scan interface.

use std::{io, path::Path};

#[test]
fn scan_is_explicitly_unimplemented() {
    let error = gracefulfs::scan(Path::new("unused")).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::Unsupported);
}
