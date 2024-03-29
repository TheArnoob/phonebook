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
        let mut phone_book = BTreeMap::new();
        for word in file_lines {
            if word == "" {
                continue;
            }
            let word_split: Vec<&str> = word.split(": ").collect();
            phone_book.insert(
                word_split[0].to_string(),
                PhoneEntry {
                    mobile: word_split[1].to_string(),
                    work: word_split[2].to_string(),
                },
            );
        }
        Ok(phone_book)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::{database::PhoneBookDB, entry::PhoneEntry};

    #[test]
    fn read_in_file() {
        let file_path = "test_file.txt";
        let phone_book = PhoneBookDB::new(file_path.into());
        let data = phone_book.read().unwrap();
        assert_eq!(data.is_empty(), true)
    }

    #[test]
    fn write_in_file() {
        let file_path = "test_file1.txt";
        let phone_book = PhoneBookDB::new(file_path.into());
        let data = phone_book.read().unwrap();
        assert_eq!(data.is_empty(), true);
        let mut map = BTreeMap::new();
        map.insert(
            "cat".to_string(),
            PhoneEntry {
                mobile: "0".to_string(),
                work: "1".to_string(),
            },
        );
        phone_book.write(map).unwrap();
        let data1 = phone_book.read().unwrap();
        assert_eq!(data1.contains_key("cat"), true);
        let entry = data1.get("cat").unwrap();
        assert_eq!(entry, &PhoneEntry {mobile: "0".to_string(), work: "1".to_string()});
        // Clean up the test file
        std::fs::remove_file(&std::path::PathBuf::from(file_path)).unwrap();
    }
}
