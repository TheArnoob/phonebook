use std::{collections::BTreeMap, fs::read, io::Write};

const FILE_NAME: &str = "file.txt";

mod entry;

use prettytable::{row, Table};

use crate::entry::PhoneEntry;

fn main() {
    loop {
        println!("Enter one of these commands:");
        let command = get_input_from_user("show, add, remove, modify, exit");
        if command == "show" {
            let phone_book = map_reader(FILE_NAME.into()).expect("Cannot read data");
            show_phone_book(&phone_book);
        } else if command == "exit" {
            return;
        } else if command == "add" {
            let name = get_input_from_user("Please enter a name");
            let mut phone_book = map_reader(FILE_NAME.into()).expect("Cannot read data");
            if phone_book.contains_key(&name) {
                println!("The name already exists.");
                continue;
            }
            let phone_number = get_input_from_user("Please enter a phone number");
            let phone_number1 = get_input_from_user("please enter another number");
            phone_book.insert(
                name,
                PhoneEntry {
                    mobile: phone_number,
                    work: phone_number1,
                },
            );
            map_writer(phone_book, FILE_NAME.into()).expect("failed to write");
        } else if command == "remove" {
            let name = get_input_from_user("Please enter a name to remove");
            let mut phone_book = map_reader(FILE_NAME.into()).unwrap();
            if phone_book.contains_key(&name) {
                phone_book.remove(&name);
                map_writer(phone_book, FILE_NAME.into()).unwrap();
                println!("Entry removed successfully")
            } else if !phone_book.contains_key(&name) {
                println!("The file dosen't contain the data");
            }
        } else if command == "modify" {
            let name = get_input_from_user("Please enter a name to modify: ");
            let mut phone_book = map_reader(FILE_NAME.into()).unwrap();
            let mutable_entry = phone_book.get_mut(&name);
            match mutable_entry {
                Some(phone_number_in_phone_book) => {
                    let new_phone_number = get_input_from_user("Please enter the new phone number");
                    let new_phone_number1 =
                        get_input_from_user("Please enter another phone number");
                    *phone_number_in_phone_book = PhoneEntry {
                        mobile: new_phone_number,
                        work: new_phone_number1,
                    };
                    map_writer(phone_book, FILE_NAME.into()).unwrap();
                }
                None => println!("the file dosen't contain the entry"),
            }
        } else {
            println!("try again")
        }
    }
}

fn map_writer(
    phone_book: BTreeMap<String, PhoneEntry>,
    file_path: std::path::PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = std::fs::File::create(file_path)?;
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

fn map_reader(
    file_path: std::path::PathBuf,
) -> Result<BTreeMap<String, PhoneEntry>, Box<dyn std::error::Error>> {
    if !file_path.exists() {
        return Ok(BTreeMap::new());
    }

    let file_bytes = read(file_path)?;
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

fn get_input_from_user(message: &str) -> String {
    println!("{message}");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();
    name
}

fn show_phone_book(phone_book: &BTreeMap<String, PhoneEntry>) {
    if !phone_book.is_empty() {
        let mut table = Table::new();
        table.add_row(row!("Name", "Mobile number", "Work number"));
        for (name, phone_entry) in phone_book {
            table.add_row(row!(name, phone_entry.mobile, phone_entry.work));
        }
        table.printstd();
    } else {
        println!("The phone book is empty.");
    }
}
