use crate::entry::PhoneEntry;
use rusqlite::{Connection, Result};
use std::{collections::BTreeMap, path::PathBuf};

pub struct PhoneBookDB {
    file_path1: PathBuf,
}

impl PhoneBookDB {
    pub fn new(file_path: std::path::PathBuf) -> PhoneBookDB {
        PhoneBookDB {
            file_path1: file_path,
        }
    }

    pub fn write_all_entries(
        &self,
        phone_book: &BTreeMap<String, PhoneEntry>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open(&self.file_path1)?;

        conn.execute("CREATE TABLE IF NOT EXISTS phone_book (name TEXT NOT NULL, phone_number TEXT NOT NULL, work_number TEXT NOT NULL)", ())?;
        // To clear the table before inserting the new entries
        conn.execute("DELETE FROM phone_book", ())?;
        for (name, phone_entry) in phone_book.iter() {
            conn.execute(
                "INSERT INTO phone_book (name, phone_number, work_number) VALUES (?1, ?2, ?3)",
                (name, &phone_entry.mobile, &phone_entry.work),
            )?;
        }

        Ok(())
    }

    pub fn read_all_entries(
        &self,
    ) -> Result<BTreeMap<String, PhoneEntry>, Box<dyn std::error::Error>> {
        let conn = Connection::open(&self.file_path1)?;
        conn.execute("CREATE TABLE IF NOT EXISTS phone_book (name TEXT NOT NULL, phone_number TEXT NOT NULL, work_number TEXT NOT NULL)", ())?;
        let mut stmt = conn.prepare("SELECT name, phone_number, work_number FROM phone_book")?;
        let phone_book_iter = stmt.query_map([], |row| {
            let name: String = row.get("name")?;
            let phone_number: String = row.get("phone_number")?;
            let work_number: String = row.get("work_number")?;

            Ok((name, phone_number, work_number))
        })?;
        let mut phone_book = BTreeMap::new();

        for phone_book_entry in phone_book_iter {
            let phone_book_entry = phone_book_entry.unwrap();
            phone_book.insert(
                phone_book_entry.0,
                PhoneEntry {
                    mobile: phone_book_entry.1,
                    work: phone_book_entry.2,
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
        let data = phone_book.read_all_entries().unwrap();
        assert_eq!(data.is_empty(), true)
    }

    #[test]
    fn write_in_file() {
        let file_path = "text_file1.txt";
        let phone_book = PhoneBookDB::new(file_path.into());
        let data = phone_book.read_all_entries().unwrap();
        assert_eq!(data.is_empty(), true);
        let mut map = BTreeMap::new();
        map.insert(
            "cat".to_string(),
            PhoneEntry {
                mobile: "0".to_string(),
                work: "1".to_string(),
            },
        );
        phone_book.write_all_entries(&map).unwrap();
        let data1 = phone_book.read_all_entries().unwrap();
        assert_eq!(data1.contains_key("cat"), true);
        let entry = data1.get("cat").unwrap();
        assert_eq!(
            entry,
            &PhoneEntry {
                mobile: "0".to_string(),
                work: "1".to_string()
            }
        );
        // Clean up the test file
        std::fs::remove_file(&std::path::PathBuf::from(file_path)).unwrap();
    }

    #[test]
    fn read_empty_file() {
        let file_path = "test_file2.txt";
        // Check if the file exists and if it exists delete it.
        if std::path::PathBuf::from(file_path).exists() {
            std::fs::remove_file(file_path).unwrap();
        }
        let phone_book = PhoneBookDB::new(file_path.into());
        // Make a new map.
        let mut map = BTreeMap::new();
        // read the phone book database.
        let data = phone_book
            .read_all_entries()
            .expect("Cannot read the data from the file.");
        // Assert that there is no data read.
        assert_eq!(data.is_empty(), true);
        map.insert(
            "Arnold".to_string(),
            PhoneEntry {
                mobile: "050343456".to_string(),
                work: "05043434332".to_string(),
            },
        );
        phone_book
            .write_all_entries(&map)
            .expect("Cannot write the map");
        map.remove_entry(&"Arnold".to_string());
        // Assert that there is no data read.
        assert_eq!(map.is_empty(), true);
        phone_book
            .write_all_entries(&map)
            .expect("Cannot write the map.");
        // read the data
        let data1 = phone_book
            .read_all_entries()
            .expect("Cannot read the data from the file.");
        // Assert that there is no data read.
        assert_eq!(data1.is_empty(), true);
        // Clean up the file.
        std::fs::remove_file(&std::path::PathBuf::from(file_path)).unwrap();
    }
}
