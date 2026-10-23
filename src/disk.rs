use std::{fs::File, io::{self, Read, Seek, SeekFrom}, path::PathBuf};
use crate::error_helpers::error_mapping;

pub(crate) fn read_at(disk_path: &PathBuf, lba: u64, buffer: &mut [u8]) -> io::Result<()> {
    let mut file = File::open(disk_path).map_err(|e| {
        error_mapping(e, io::ErrorKind::NotFound, disk_path, "file does not exist")
    })?;
    file.seek(SeekFrom::Start(lba * 512))?;

    file.read_exact(buffer).map_err(|e| {
        let msg = format!("file smaller than {} bytes", buffer.len());
        error_mapping(e, io::ErrorKind::UnexpectedEof, disk_path, &msg)
    })
}

pub(crate) fn get_size(num_as_bytes: u64) -> String {
    const KIB: u64 = 1 << 10;
    const MIB: u64 = 1 << 20;
    const GIB: u64 = 1 << 30;
    const TIB: u64 = 1 << 40;
    if num_as_bytes >= TIB {
        return format!("{}T", num_as_bytes / TIB);
    } else if num_as_bytes >= GIB {
        return format!("{}G", num_as_bytes / GIB);
    } else if num_as_bytes >= MIB {
        return format!("{}M", num_as_bytes / MIB);
    } else {
        return format!("{}K", num_as_bytes / KIB);
    }
}