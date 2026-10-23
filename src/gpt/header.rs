use std::io;

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

