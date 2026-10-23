use std::{path, io};
use crate::{
    mbr_entry::MbrEntry, disk, table::Table
};

pub struct MbrParser {
    disk_path: path::PathBuf,
    contents: Vec<u8>
}

impl Table for MbrParser {
    type Entry = MbrEntry;
}

impl MbrParser {
    pub fn new(disk_path: path::PathBuf) -> io::Result<Self> {
        let mut buffer = vec![0u8; 512];
        disk::read_at(&disk_path, 0, &mut buffer)?;
        if !Self::is_mbr(&buffer) {
            return Err(
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("'{}': no valid mbr signature found", &disk_path.display())
                )
            );
        }

        Ok(Self {
            disk_path,
            contents: buffer
        })
    }

    fn is_mbr(buffer: &[u8]) -> bool {
        buffer.len() >= 512 && (buffer[510] == 0x55 && buffer[511] == 0xAA)
    }

    fn table_headings() -> Vec<&'static str> {
        vec!["Device", "Boot", "Start", "End", "Sectors", "Size", "Id", "Type"]
    }

    pub fn run(&self) -> io::Result<()> {
        let buffer = &self.contents;
        let table_headings: Vec<&str> = Self::table_headings();

        let mut entries = Vec::<MbrEntry>::new();
        let partition_table = &buffer[446..446+64];

        // Responsible for generating empty TABLE_HEADINGS list
        for (index, row) in partition_table.chunks_exact(16).enumerate() {
            let entry = MbrEntry::new(
                row.try_into().unwrap(),
                format!("{}", self.disk_path.display()),
                index + 1,
            );
            if entry.disk_type != 0x00 {
                entries.push(entry);
            }
        }

        // Responsible for printing table
        if !entries.is_empty() {
            Self::display_table(&table_headings, &entries);
        }

        Ok(())
    }
}