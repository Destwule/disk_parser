use std::{io, path::PathBuf};

pub(crate) fn error_mapping(err: io::Error, error_kind: io::ErrorKind, disk_path: &PathBuf, msg: &str) -> io::Error {
    let path = disk_path.display();
    if err.kind() == error_kind {
        io::Error::new(err.kind(), format!("'{path}': {msg}"))
    } else {
        io::Error::new(err.kind(), format!("'{path}': {err}"))
    }
}

pub fn general_errors(err: io::Error, disk_path: &PathBuf) -> io::Error {
    match err.kind() {
        io::ErrorKind::InvalidData => {
            io::Error::new(err.kind(), format!("cannot open {}: Invalid argument", disk_path.display()))
        }
        _ => err
    }
}