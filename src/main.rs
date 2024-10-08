const FILE_NAME: &str = "file.sqlite";
use std::collections::BTreeMap;
mod database;
mod entry;

use crate::entry::PhoneEntry;
use prettytable::{row, Table};
fn main() {
    loop {
        let phone_book_db =
            database::PhoneBookDB::new(Some(FILE_NAME.into())).expect("Failed to open file.");
        println!("Please enter one of these commands:");
        let command = get_input_from_user("show, add, remove, modify, exit");
        if command == "show" {
            let phone_book = phone_book_db.read_all_entries().expect("Cannot read data");
            show_phone_book(&phone_book);
        } else if command == "exit" {
            return;
        } else if command == "add" {
            let name = get_input_from_user("Please enter a name");
            let mut phone_book = phone_book_db
                .read_all_entries()
                .expect("Cannot find the file.");
            if phone_book.contains_key(&name) {
                println!("The name already exists.");
                continue;
            }
            let phone_number = get_input_from_user("Please enter a phone number");
            let phone_number1 = get_input_from_user("please enter another number");
            phone_book.insert(
                name.clone(),
                PhoneEntry {
                    mobile: phone_number.clone(),
                    work: phone_number1.clone(),
                },
            );
            phone_book_db
                .write_entry(
                    name,
                    PhoneEntry {
                        mobile: phone_number,
                        work: phone_number1,
                    },
                )
                .expect("Cannot write data");
        } else if command == "remove" {
            let name = get_input_from_user("Please enter a name to remove");
            let phone_book = phone_book_db
                .read_all_entries()
                .expect("Cannot read the data from the file.");
            if phone_book.contains_key(&name) {
                phone_book_db.remove_entry(&name).unwrap();
                println!("Entry removed successfully")
            } else if !phone_book.contains_key(&name) {
                println!("The file dosen't contain the data");
            }
        } else if command == "modify" {
            let name = get_input_from_user("Please enter a name to modify: ");

            if phone_book_db.read_entry(name.clone()).unwrap().is_some() {
                let new_phone_number = get_input_from_user("Please enter the new phone number");
                let new_phone_number1 = get_input_from_user("Please enter another phone number");
                phone_book_db
                    .modify_entry(
                        name,
                        PhoneEntry {
                            mobile: new_phone_number,
                            work: new_phone_number1,
                        },
                    )
                    .unwrap();
            } else {
                println!("The name doesen't exist.")
            }
        } else {
            println!("try again")
        }
    }
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
        phone_book.into_iter().for_each(|(name, phone_entry)| {
            table.add_row(row!(name, phone_entry.mobile, phone_entry.work));
        });
        table.printstd()
    } else {
        println!("The phone book is empty.");
    }
}
