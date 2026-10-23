use crate::{disk, table};
use std::{io, path};

pub struct GptHeader {  
    signature: [u8; 8],
    entry_array_start_lba: u64,
    number_of_entries: u32,
    size_of_each_entry: u32
}

impl GptHeader {
    pub fn new(bytes: &[u8]) -> io::Result<Self> {
        let signature: [u8; 8] = bytes[..8].try_into().unwrap();
        
        if &signature != b"EFI PART" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData, "no valid gpt signature found"
            ))
        }

        let entry_array_start_lba = u64::from_le_bytes(bytes[72..80].try_into().unwrap());
        let number_of_entries = u32::from_le_bytes(bytes[80..84].try_into().unwrap());
        let size_of_each_entry = u32::from_le_bytes(bytes[84..88].try_into().unwrap());
    
        Ok(Self {
            signature,
            entry_array_start_lba,
            number_of_entries,
            size_of_each_entry
        })
    }

    pub fn signature(&self) -> &[u8; 8] { &self.signature }
    pub fn entry_start_lba(&self) -> u64 { self.entry_array_start_lba}
    pub fn each_entry_size(&self) -> u32 { self.size_of_each_entry }

    pub fn total_entry_array_len(&self) -> u64 {
        self.number_of_entries as u64 * self.size_of_each_entry as u64
    }

    pub fn to_row(&self) -> Vec<String> {
        vec![str::from_utf8(&self.signature).unwrap().to_string(), format!("{}", self.entry_array_start_lba), format!("{}", self.number_of_entries), format!("{}", self.size_of_each_entry)]
    }
}


pub struct GptEntry {
    number: usize,
    pub device: String,
    pub start_lba: u64,
    pub end_lba: u64,
    pub sectors: u64,
    pub size: u64,
    pub guid: [u8; 16],
    is_empty: bool,
}

impl table::Row for GptEntry {
    fn to_row(&self) -> Vec<String> {
        let device = format!("{}{}",  self.device, self.number);
        let start_lba = format!("{}", self.start_lba);
        let end_lba = format!("{}", self.end_lba);
        let sectors = format!("{}", self.sectors);
        let size = disk::get_size(self.size);
        
        let inter = Self::guid_to_string(&self.guid);
        let gpt_type = Self::gpt_type(inter.as_str());

        vec![device, start_lba, end_lba, sectors, size, gpt_type]
    }
}

impl GptEntry {
    pub fn new(row: &[u8], device: &path::PathBuf, number: usize) -> Self {
        let device = format!("{}", device.display());
        let start_lba = u64::from_le_bytes(row[32..40].try_into().unwrap());
        let end_lba = u64::from_le_bytes(row[40..48].try_into().unwrap());
        let sectors = end_lba - start_lba + 1;
        let size = sectors * 512;
        let guid = &row[..16];
        let is_empty = guid[0..16].iter().all(|&b| b == 0);
        Self {
            number,
            device,
            start_lba,
            end_lba,
            sectors,
            size,
            is_empty,
            guid: guid.try_into().unwrap(),
        }
    }

    pub fn is_empty(&self) -> bool { self.is_empty }

    fn guid_to_string(b: &[u8]) -> String {
        format!(
            "{:02X}{:02X}{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
            b[3], b[2], b[1], b[0],   // group 1: first 4 bytes, REVERSED (little-endian)
            b[5], b[4],               // group 2: next 2 bytes, REVERSED
            b[7], b[6],               // group 3: next 2 bytes, REVERSED
            b[8], b[9],               // group 4: as-is
            b[10], b[11], b[12], b[13], b[14], b[15], // group 5: as-is
        )
    }

    fn gpt_type(guid: &str) -> String {
        match guid {
            "C12A7328-F81F-11D2-BA4B-00A0C93EC93B" => String::from("EFI System"),
            "0FC63DAF-8483-4772-8E79-3D69D8477DE4" => String::from("Linux filesystem"),
            "0657FD6D-A4AB-43C4-84E5-0933C84B4F4F" => String::from("Linux swap"),
            "EBD0A0A2-B9E5-4433-87C0-68B6B72699C7" => String::from("Microsoft basic data"),
            "21686148-6449-6E6F-744E-656564454649" => String::from("BIOS boot"),
            other => format!("Unknown ({other})"), // same idea as your MBR fallback
        }
    }
}