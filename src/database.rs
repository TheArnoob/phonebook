use crate::entry::PhoneEntry;
use rusqlite::{params, Connection, Result};
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

    fn create_table_if_not_exists(&self) -> Result<()> {
        let conn = Connection::open(&self.file_path1)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS
         phone_book (name TEXT NOT NULL, phone_number TEXT NOT NULL, work_number TEXT NOT NULL)",
            (),
        )?;

        Ok(())
    }

    pub fn write_all_entries(
        &self,
        phone_book: &BTreeMap<String, PhoneEntry>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.create_table_if_not_exists()?;

        let conn = Connection::open(&self.file_path1)?;

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
        let data = self.read_all_entries_as_vec(None)?;

        let phone_book = data.into_iter().collect();
        Ok(phone_book)
    }

    pub fn write_entry(
        &self,
        name: String,
        entry: PhoneEntry,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.create_table_if_not_exists()?;

        let conn = Connection::open(&self.file_path1)?;
        conn.execute("DELETE FROM phone_book WHERE name = ?1", [&name])?;

        conn.execute(
            "INSERT INTO phone_book (name, phone_number, work_number) VALUES(?1, ?2, ?3)",
            (name, entry.mobile, entry.work),
        )?;

        Ok(())
    }

    /// Searches for names of the entries with the name you give it
    /// If name is None it will return all entries
    /// And if the name is Some(String::from(...)),
    /// it will for the search names of the entries with the name you give it in the database
    fn read_all_entries_as_vec(
        &self,
        name: Option<String>,
    ) -> Result<Vec<(String, PhoneEntry)>, Box<dyn std::error::Error>> {
        self.create_table_if_not_exists()?;

        let conn = Connection::open(&self.file_path1)?;

        let mut stmt = match &name {
            Some(_name) => conn.prepare(
                "SELECT name, phone_number, work_number FROM phone_book WHERE name = ?1",
            )?,
            None => conn.prepare("SELECT name, phone_number, work_number FROM phone_book")?,
        };

        let params = match &name {
            Some(name) => params![name.clone()],
            None => params![],
        };

        let phone_book_iter = stmt.query_map(params, |row| {
            let name: String = row.get("name")?;
            let phone_number: String = row.get("phone_number")?;
            let work_number: String = row.get("work_number")?;

            Ok((name, phone_number, work_number))
        })?;
        let mut phone_book = Vec::new();

        for phone_book_entry in phone_book_iter {
            let phone_book_entry = phone_book_entry.unwrap();
            phone_book.push((
                phone_book_entry.0,
                PhoneEntry {
                    mobile: phone_book_entry.1,
                    work: phone_book_entry.2,
                },
            ));
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

    #[test]
    fn single_writes() {
        let file_path = "test_file3.txt";

        let phone_book_db = PhoneBookDB::new(file_path.into());
        phone_book_db
            .write_entry(
                "Arnold".to_owned(),
                PhoneEntry {
                    mobile: "9027590".to_owned(),
                    work: "3795780357".to_owned(),
                },
            )
            .unwrap();
        phone_book_db
            .write_entry(
                "Maram".to_owned(),
                PhoneEntry {
                    mobile: "02875902".to_owned(),
                    work: "98270987".to_owned(),
                },
            )
            .unwrap();

        phone_book_db
            .write_entry(
                "Samer".to_owned(),
                PhoneEntry {
                    mobile: "375946".to_owned(),
                    work: "738749".to_owned(),
                },
            )
            .unwrap();

        let read_phone_book_db = phone_book_db.read_all_entries_as_vec(None).unwrap();
        assert!(read_phone_book_db.contains(&(
            "Arnold".to_owned(),
            PhoneEntry {
                mobile: "9027590".to_owned(),
                work: "3795780357".to_owned(),
            },
        )));
        assert!(read_phone_book_db.contains(&(
            "Maram".to_owned(),
            PhoneEntry {
                mobile: "02875902".to_owned(),
                work: "98270987".to_owned(),
            },
        )));
    }

    #[test]
    fn writes_then_reads() {
        let file_path = "test_file_4";
        let _ = std::fs::remove_file(&std::path::PathBuf::from(file_path));
        let phone_book_db = PhoneBookDB::new(file_path.into());

        assert_eq!(phone_book_db.read_all_entries_as_vec(None).unwrap(), vec![]);

        phone_book_db
            .write_entry(
                "arnold".to_owned(),
                PhoneEntry {
                    mobile: "345345".to_owned(),
                    work: "3535345".to_owned(),
                },
            )
            .unwrap();

        assert_eq!(
            phone_book_db.read_all_entries_as_vec(None).unwrap(),
            vec![(
                "arnold".to_owned(),
                PhoneEntry {
                    mobile: "345345".to_owned(),
                    work: "3535345".to_owned(),
                },
            )]
        );

        assert_eq!(
            phone_book_db
                .read_all_entries_as_vec(Some(String::from("arnold")))
                .unwrap(),
            vec![(
                "arnold".to_owned(),
                PhoneEntry {
                    mobile: "345345".to_owned(),
                    work: "3535345".to_owned(),
                },
            )]
        );

        phone_book_db
            .write_entry(
                "Maram".to_owned(),
                PhoneEntry {
                    mobile: "9870982".to_owned(),
                    work: "279573".to_owned(),
                },
            )
            .unwrap();

        assert_eq!(
            phone_book_db.read_all_entries_as_vec(None).unwrap(),
            vec![
                (
                    "arnold".to_owned(),
                    PhoneEntry {
                        mobile: "345345".to_owned(),
                        work: "3535345".to_owned(),
                    },
                ),
                (
                    "Maram".to_owned(),
                    PhoneEntry {
                        mobile: "9870982".to_owned(),
                        work: "279573".to_owned(),
                    },
                )
            ]
        );

        assert_eq!(
            phone_book_db
                .read_all_entries_as_vec(Some(String::from("arnold")))
                .unwrap(),
            vec![(
                "arnold".to_owned(),
                PhoneEntry {
                    mobile: "345345".to_owned(),
                    work: "3535345".to_owned(),
                },
            )]
        );

        assert_eq!(
            phone_book_db
                .read_all_entries_as_vec(Some(String::from("Maram")))
                .unwrap(),
            vec![(
                "Maram".to_owned(),
                PhoneEntry {
                    mobile: "9870982".to_owned(),
                    work: "279573".to_owned(),
                },
            )]
        )
    }

    #[test]
    fn unique_names() {
        let file_path = "test_file5";
        let _ = std::fs::remove_file(&std::path::PathBuf::from(file_path));
        let phone_book_db = PhoneBookDB::new(file_path.into());

        phone_book_db
            .write_entry(
                "Arnold".to_owned(),
                PhoneEntry {
                    mobile: "397097345".to_owned(),
                    work: "789346535".to_owned(),
                },
            )
            .unwrap();

        phone_book_db
            .write_entry(
                "Arnold".to_owned(),
                PhoneEntry {
                    mobile: "983534354".to_owned(),
                    work: "34759384793".to_owned(),
                },
            )
            .unwrap();

        assert_eq!(
            phone_book_db.read_all_entries_as_vec(None).unwrap(),
            vec![(
                "Arnold".to_owned(),
                PhoneEntry {
                    mobile: "983534354".to_owned(),
                    work: "34759384793".to_owned()
                }
            )]
        )
    }
}
