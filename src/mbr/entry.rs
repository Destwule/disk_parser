use crate::{disk, table};

pub struct MbrEntry {
    pub entry_name: String,
    pub number: usize,
    pub bootable: bool,
    pub start_lba: u32,
    pub end_lba: u64,
    pub sectors: u32,
    pub id: u8,
    pub size: u64,
    pub disk_type: u8
}


impl table::Row for MbrEntry {
    fn to_row(&self) -> Vec<String> {
        let device = format!("{}{}", self.entry_name, self.number);
        let boot = if self.bootable { "true" } else { "false" };
        let start = format!("{}", &self.start_lba);
        let sectors = format!("{}", &self.sectors);
        let end = format!("{}", &self.end_lba);
        let size = disk::get_size(self.size);
        let id = format!("{:x}", self.id);
        let disk_type = Self::disk_type(self.disk_type);

        vec![device, format!("{boot}"), start, end, sectors, size, id, disk_type]
    }
}

impl MbrEntry {
    pub fn new(row: [u8; 16], device: String, number: usize) -> Self {
        let entry_name: String = device;
        let bootable = row[0] == 0x80;
        let id = row[4];
        let start_lba = u32::from_le_bytes(row[8..12].try_into().unwrap());
        let sectors = u32::from_le_bytes(row[12..16].try_into().unwrap());
        let end_lba = (start_lba as u64 + sectors as u64).saturating_sub(1);
        let size = sectors as u64 * 512;
        let disk_type = id;
        MbrEntry {
            entry_name,
            number,
            bootable,
            start_lba,
            end_lba,
            sectors,
            size,
            id,
            disk_type,
        }
    }

    pub fn disk_type(num: u8) -> String {
        match num {
            0x00 => String::from("Empty"),
            0x01 => String::from("FAT12"),
            0x04 => String::from("FAT16 (small, under 32 MB)"),
            0x06 => String::from("FAT16 (larger)"),
            0x0E => String::from("FAT16 (LBA)"),
            0x0B => String::from("FAT32 (CHS)"),
            0x0C => String::from("FAT32 (LBA)"),
            0x07 => String::from("NTFS (also exFAT)"),
            0x05 => String::from("Extended partition (CHS)"),
            0x0F => String::from("Extended partition (LBA)"),
            0x82 => String::from("Linux swap"),
            0x83 => String::from("Linux filesystem (ext2/3/4, XFS, Btrfs...)"),
            0x8E => String::from("Linux LVM"),
            0xFD => String::from("Linux RAID"),
            0xAF => String::from("Apple HFS/HFS+"),
            0xEF => String::from("EFI System Partition"),
            0xEE => String::from("GPT protective MBR"),
            value => format!("'0x{value:x}' is not a supported type")
        }
    }
}