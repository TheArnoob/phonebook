const FILE_NAME: &str = "file.txt";
use std::collections::BTreeMap;
mod database;
mod entry;

use crate::entry::PhoneEntry;
use prettytable::{row, Table};
fn main() {
    loop {
        let phone_book_db = database::PhoneBookDB::new(FILE_NAME.into());
        println!("Please enter one of these commands:");
        let command = get_input_from_user("show, add, remove, modify, exit");
        if command == "show" {
            let phone_book = phone_book_db.read().expect("Cannot read data");
            show_phone_book(&phone_book);
        } else if command == "exit" {
            return;
        } else if command == "add" {
            let name = get_input_from_user("Please enter a name");
            let mut phone_book = phone_book_db.read().expect("Cannot find the file.");
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
            phone_book_db.write(phone_book).expect("Cannot write data");
        } else if command == "remove" {
            let name = get_input_from_user("Please enter a name to remove");
            let mut phone_book = phone_book_db
                .read()
                .expect("Cannot read the data from the file.");
            if phone_book.contains_key(&name) {
                phone_book.remove(&name);
                phone_book_db.write(phone_book).expect("Cannot write data");
                println!("Entry removed successfully")
            } else if !phone_book.contains_key(&name) {
                println!("The file dosen't contain the data");
            }
        } else if command == "modify" {
            let name = get_input_from_user("Please enter a name to modify: ");
            let mut phone_book = phone_book_db
                .read()
                .expect("Cannot read the data from the file.");
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
                    phone_book_db.write(phone_book).expect("Cannot write data");
                }
                None => println!("the file dosen't contain the entry"),
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
        for (name, phone_entry) in phone_book {
            table.add_row(row!(name, phone_entry.mobile, phone_entry.work));
        }
        table.printstd();
    } else {
        println!("The phone book is empty.");
    }
}
