use std::{collections::BTreeMap, fs::read, io::Write};

const FILE_NAME: &str = "file.txt";

fn main() {
    loop {
        println!("Enter one of these commands:");
        println!("show, exit, add, remove, modify");
        let mut commands = String::new();
        std::io::stdin().read_line(&mut commands).unwrap();
        let parsed_command = commands.trim().parse::<String>().unwrap();
        if parsed_command == "show" {
            let phone_book = map_reader(FILE_NAME.into()).expect("Cannot read data");
            println!("{phone_book:?}");
        } else if parsed_command == "exit" {
            return;
        } else if parsed_command == "add" {
            println!("please enter a name");
            let mut name = String::new();
            std::io::stdin().read_line(&mut name).unwrap();
            println!("please enter a number");
            let mut phone_number = String::new();
            std::io::stdin().read_line(&mut phone_number).unwrap();
            let mut phone_book = map_reader(FILE_NAME.into()).expect("Cannot read data");
            phone_book.insert(name.trim().to_string(), phone_number.trim().to_string());
            map_writer(phone_book, FILE_NAME.into()).expect("failed to write");
        } else if parsed_command == "remove" {
            println!("Please enter a name");
            let mut name: String = String::new();
            std::io::stdin().read_line(&mut name).unwrap();
            let name = name.trim().to_string();
            let mut phone_book = map_reader(FILE_NAME.into()).unwrap();
            if phone_book.contains_key(&name) {
                phone_book.remove(&name);
                map_writer(phone_book, FILE_NAME.into()).unwrap();
                println!("Entry removed successfully")
            } else if !phone_book.contains_key(&name) {
                println!("The file dosen't contain the data");
            }
        } else if parsed_command == "modify" {
            println!("Please enter a name to modify:");
            let mut name = String::new();
            std::io::stdin().read_line(&mut name).unwrap();
            let name = name.trim().to_string();
            let mut phone_book = map_reader(FILE_NAME.into()).unwrap();
            println!("Please enter the new number: ");
            let mut new_phone_number = String::new();
            std::io::stdin().read_line(&mut new_phone_number).unwrap();
            let new_phone_number = new_phone_number.trim().to_string();
            let mutable_entry = phone_book.get_mut(&name);
            match mutable_entry {
                Some(phone_number_in_phone_book) => *phone_number_in_phone_book = new_phone_number,
                None => println!("the file dosen't contain the entry"),
            }
            map_writer(phone_book, FILE_NAME.into()).unwrap();
        } else {
            println!("try again")
        }
    }
}

fn map_writer(
    phone_book: BTreeMap<String, String>,
    file_path: std::path::PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = std::fs::File::create(file_path)?;
    for (i, (name, phone_number)) in phone_book.iter().enumerate() {
        file.write_all(name.as_bytes())?;
        file.write_all(": ".as_bytes())?;
        file.write_all(phone_number.as_bytes())?;
        if i != phone_book.len() - 1 {
            file.write_all("\n".as_bytes())?;
        }
    }

    Ok(())
}

fn map_reader(file_path: std::path::PathBuf) -> Result<BTreeMap<String, String>, Box<dyn std::error::Error>> {
    if !file_path.exists() {
        return Ok(BTreeMap::new());
    }

    let file_bytes = read(file_path)?;
    let file_as_string = String::from_utf8(file_bytes)?;
    let file_lines: Vec<&str> = file_as_string.split("\n").collect();
    let mut map = BTreeMap::new();
    for word in file_lines {
        let word_split: Vec<&str> = word.split(": ").collect();
        map.insert(word_split[0].to_string(), word_split[1].to_string());
    }
    Ok(map)
}
