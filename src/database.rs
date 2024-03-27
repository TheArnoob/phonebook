use crate::entry::PhoneEntry;
use std::{collections::BTreeMap, fs::read, io::Write, path::PathBuf};
pub struct PhoneBookDB {
    file_path1: PathBuf,
}

impl PhoneBookDB {
    pub fn new(file_path: std::path::PathBuf) -> PhoneBookDB {
        PhoneBookDB {
            file_path1: file_path,
        }
    }
    pub fn write(
        &self,
        phone_book: BTreeMap<String, PhoneEntry>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = std::fs::File::create(&self.file_path1)?;
        for (i, (name, phone_entry)) in phone_book.iter().enumerate() {
            file.write_all(name.as_bytes())?;
            file.write_all(": ".as_bytes())?;
            file.write_all(phone_entry.mobile.as_bytes())?;
            file.write_all(": ".as_bytes())?;
            file.write_all(phone_entry.work.as_bytes())?;
            if i != phone_book.len() - 1 {
                file.write_all("\n".as_bytes())?;
            }
        }

        Ok(())
    }

    pub fn read(&self) -> Result<BTreeMap<String, PhoneEntry>, Box<dyn std::error::Error>> {
        if !self.file_path1.exists() {
            return Ok(BTreeMap::new());
        }

        let file_bytes = read(&self.file_path1)?;
        let file_as_string = String::from_utf8(file_bytes)?;
        let file_lines: Vec<&str> = file_as_string.split("\n").collect();
        let mut map = BTreeMap::new();
        for word in file_lines {
            if word == "" {
                continue;
            }
            let word_split: Vec<&str> = word.split(": ").collect();
            map.insert(
                word_split[0].to_string(),
                PhoneEntry {
                    mobile: word_split[1].to_string(),
                    work: word_split[2].to_string(),
                },
            );
        }
        Ok(map)
    }
}
