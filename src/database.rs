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

    pub fn modify_entry(
        &self,
        name: String,
        entry: PhoneEntry,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.create_table_if_not_exists()?;
        let conn = Connection::open(&self.file_path1)?;
        conn.execute(
            "UPDATE phone_book SET phone_number = ?2, work_number = ?3 WHERE name = ?1",
            [&name, &entry.mobile, &entry.work],
        )?;

        Ok(())
    }

    pub fn remove_entry(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.create_table_if_not_exists()?;
        let conn = Connection::open(&self.file_path1)?;

        conn.execute("DELETE FROM phone_book WHERE name = ?1", [name])?;
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

    pub fn read_entry(
        &self,
        name: String,
    ) -> Result<Option<PhoneEntry>, Box<dyn std::error::Error>> {
        let data = self.read_all_entries_as_vec(Some(name))?;
        if data.is_empty() {
            Ok(None)
        } else {
            Ok(Some(data[0].1.clone()))
        }
    }

    /// Searches for names of the entries with the name you give it.
    /// If name is None it will return all entries.
    /// And if the name is Some(String::from(...)),
    /// it will for the search names of the entries with the name you give it in the database.
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
    use crate::{database::PhoneBookDB, entry::PhoneEntry};

    #[test]
    fn read_in_file() {
        let file_path = "test_file.sqlite";
        let phone_book = PhoneBookDB::new(file_path.into());
        let data = phone_book.read_all_entries().unwrap();
        assert_eq!(data.is_empty(), true)
    }

    #[test]
    fn single_writes() {
        let file_path = "test_file3.sqlite";

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
                "Jack".to_owned(),
                PhoneEntry {
                    mobile: "02875902".to_owned(),
                    work: "98270987".to_owned(),
                },
            )
            .unwrap();

        phone_book_db
            .write_entry(
                "Mark".to_owned(),
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
            "Jack".to_owned(),
            PhoneEntry {
                mobile: "02875902".to_owned(),
                work: "98270987".to_owned(),
            },
        )));
    }

    #[test]
    fn writes_then_reads() {
        let file_path = "test_file_4.sqlite";
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
                "Jack".to_owned(),
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
                    "Jack".to_owned(),
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
                .read_all_entries_as_vec(Some(String::from("Jack")))
                .unwrap(),
            vec![(
                "Jack".to_owned(),
                PhoneEntry {
                    mobile: "9870982".to_owned(),
                    work: "279573".to_owned(),
                },
            )]
        )
    }

    #[test]
    fn unique_names() {
        let file_path = "test_file5.sqlite";
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

    #[test]
    fn modify_entries() {
        let file_path = "test_file6.sqlite";
        let _ = std::fs::remove_file(&std::path::PathBuf::from(file_path));

        let phone_book_db = PhoneBookDB::new(file_path.into());

        phone_book_db
            .write_entry(
                "Arnold".to_owned(),
                PhoneEntry {
                    mobile: "83749876389".to_owned(),
                    work: "3758937498".to_owned(),
                },
            )
            .unwrap();

        phone_book_db
            .write_entry(
                "Jack".to_owned(),
                PhoneEntry {
                    mobile: "938759834".to_owned(),
                    work: "73598739074".to_owned(),
                },
            )
            .unwrap();

        phone_book_db
            .modify_entry(
                "Arnold".to_owned(),
                PhoneEntry {
                    mobile: "938759834".to_owned(),
                    work: "73598739074".to_owned(),
                },
            )
            .unwrap();

        assert_eq!(
            phone_book_db
                .read_all_entries_as_vec(Some(String::from("Arnold")))
                .unwrap(),
            vec![(
                "Arnold".to_owned(),
                PhoneEntry {
                    mobile: "938759834".to_owned(),
                    work: "73598739074".to_owned(),
                },
            )]
        );
    }
    #[test]
    fn modify_entries_not_exist() {
        let file_path = "test_file7.sqlite";

        let _ = std::fs::remove_file(&std::path::PathBuf::from(file_path));

        let phone_book_db = PhoneBookDB::new(file_path.into());
        phone_book_db
            .write_entry(
                "Jack".to_owned(),
                PhoneEntry {
                    mobile: "938759834".to_owned(),
                    work: "73598739074".to_owned(),
                },
            )
            .unwrap();

        phone_book_db
            .modify_entry(
                "Arnold".to_owned(),
                PhoneEntry {
                    mobile: "938759834".to_owned(),
                    work: "73598739074".to_owned(),
                },
            )
            .unwrap();

        assert_eq!(
            phone_book_db
                .read_all_entries_as_vec(Some(String::from("Arnold")))
                .unwrap(),
            vec![]
        );
    }

    #[test]
    fn writes_then_removes() {
        let file_path = "test_file8.sqlite";
        let _ = std::fs::remove_file(&std::path::PathBuf::from(file_path));

        let phone_book_db = PhoneBookDB::new(file_path.into());

        assert_eq!(phone_book_db.read_all_entries_as_vec(None).unwrap(), vec![]);

        phone_book_db.remove_entry("Arnold").unwrap();
        assert_eq!(phone_book_db.read_all_entries_as_vec(None).unwrap(), vec![]);

        assert_eq!(
            phone_book_db
                .read_all_entries_as_vec(Some(String::from("Arnold")))
                .unwrap(),
            vec![]
        );
        phone_book_db
            .write_entry(
                "Arnold".to_owned(),
                PhoneEntry {
                    mobile: "83750893475".to_owned(),
                    work: "738765987364".to_owned(),
                },
            )
            .unwrap();

        phone_book_db
            .write_entry(
                "Jack".to_owned(),
                PhoneEntry {
                    mobile: "3535345345".to_owned(),
                    work: "3453534562".to_owned(),
                },
            )
            .unwrap();

        assert_eq!(
            phone_book_db.read_all_entries_as_vec(None).unwrap(),
            vec![
                (
                    "Arnold".to_owned(),
                    PhoneEntry {
                        mobile: "83750893475".to_owned(),
                        work: "738765987364".to_owned(),
                    },
                ),
                (
                    "Jack".to_owned(),
                    PhoneEntry {
                        mobile: "3535345345".to_owned(),
                        work: "3453534562".to_owned(),
                    },
                )
            ]
        );

        phone_book_db.remove_entry("Arnold").unwrap();

        assert_eq!(
            phone_book_db.read_all_entries_as_vec(None).unwrap(),
            vec![(
                "Jack".to_owned(),
                PhoneEntry {
                    mobile: "3535345345".to_owned(),
                    work: "3453534562".to_owned(),
                }
            )]
        );

        assert_eq!(
            phone_book_db
                .read_all_entries_as_vec(Some(String::from("Arnold")))
                .unwrap(),
            vec![]
        );

        phone_book_db.remove_entry("Jack").unwrap();

        assert_eq!(phone_book_db.read_all_entries_as_vec(None).unwrap(), vec![]);

        assert_eq!(
            phone_book_db
                .read_all_entries_as_vec(Some(String::from("Arnold")))
                .unwrap(),
            vec![]
        );
        assert_eq!(
            phone_book_db
                .read_all_entries_as_vec(Some(String::from("Jack")))
                .unwrap(),
            vec![]
        );
    }

    #[test]
    fn writes_then_single_reads() {
        let file_path = "test_file9.sqlite";
        let phone_book_db = PhoneBookDB::new(file_path.into());

        phone_book_db
            .write_entry(
                "Arnold".to_owned(),
                PhoneEntry {
                    mobile: "903795".to_owned(),
                    work: "89347509".to_owned(),
                },
            )
            .unwrap();

        phone_book_db
            .write_entry(
                "Jack".to_owned(),
                PhoneEntry {
                    mobile: "37597343".to_owned(),
                    work: "398745".to_owned(),
                },
            )
            .unwrap();

        assert_eq!(
            phone_book_db
                .read_entry("Arnold".to_owned())
                .unwrap()
                .unwrap(),
            PhoneEntry {
                mobile: "903795".to_owned(),
                work: "89347509".to_owned(),
            },
        );

        assert_eq!(
            phone_book_db
                .read_entry("Jack".to_owned())
                .unwrap()
                .unwrap(),
            PhoneEntry {
                mobile: "37597343".to_owned(),
                work: "398745".to_owned(),
            }
        );

        assert!(phone_book_db
            .read_entry("Mark".to_owned())
            .unwrap()
            .is_none());
    }
}
