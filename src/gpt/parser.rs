use std::{path, io};
use crate::{disk, gpt::{GptHeader, GptEntry}, error_helpers::error_mapping};
use crate::{table::Table};

pub struct GptParser {
    disk_path: path::PathBuf,
    gpt_header: GptHeader,
}

impl Table for GptParser {
    type Entry = GptEntry;
}

impl GptParser {
    pub fn new(disk_path: path::PathBuf) -> io::Result<Self> {
        let mut buffer = [0u8; 512];
        
        disk::read_at(&disk_path, 1, &mut buffer)?;
        let gpt_header = GptHeader::new(&buffer).map_err(|e| {
            let msg = e.to_string();
            error_mapping(e, io::ErrorKind::InvalidData, &disk_path, &msg)
        })?;

        Ok(Self {
            disk_path,
            gpt_header
        })
    }

    pub fn table_headings() -> Vec<&'static str> {
        vec!["Device", "Start", "End", "Sectors", "Size", "Type"]
    }

    
    pub fn run(&self) -> io::Result<()> {
        let entry_start_lba = self.gpt_header.entry_start_lba();
        let total_entry_array_len: usize = self.gpt_header.total_entry_array_len() as usize;
        let mut buffer = vec![0u8; total_entry_array_len];
        disk::read_at(&self.disk_path, entry_start_lba, &mut buffer)?;
        
        let mut entries: Vec<GptEntry> = Vec::new();
        let table_headings: Vec<&str> = Self::table_headings();
        
        let each_entry_size = self.gpt_header.each_entry_size() as usize;
        
        for (index, chunk) in buffer.chunks(each_entry_size).enumerate() {
            let entry = GptEntry::new(chunk, &self.disk_path, index+1);
            
            if !entry.is_empty() {
                entries.push(entry);
            }
        }
    
        // This section prints the entire table
        if !entries.is_empty() {
            Self::display_table(&table_headings, &entries);
        }

        Ok(())
    }
}