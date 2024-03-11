use std::{collections::BTreeMap, fs::read, io::Write,};

const FILE_NAME: &str = "file.txt";

fn main() {
    loop {
        println!("Enter one of these commands:");
        println!("show, exit, add, remove");
        let mut commands = String::new();
        std::io::stdin().read_line(&mut commands).unwrap();
        let parsed_command = commands.trim().parse::<String>().unwrap();
        if parsed_command == "show" {
            let phone_book = map_reader(FILE_NAME.to_string()).expect("Cannot read data");
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
            let mut phone_book = map_reader(FILE_NAME.to_string()).expect("Cannot read data");
            phone_book.insert(name.trim().to_string(), phone_number.trim().to_string());
            map_writer(phone_book, FILE_NAME.to_string()).expect("failed to write");
        } else if parsed_command == "remove" {
            println!("Please enter a name");
            let mut name: String = String::new();
            std::io::stdin().read_line(&mut name).unwrap();
            let name = name.trim().to_string();
            let mut phone_book = map_reader(FILE_NAME.to_string()).unwrap();
            if phone_book.contains_key(&name) {
                phone_book.remove(&name);
                map_writer(phone_book, FILE_NAME.to_string()).unwrap();
                println!("Entry removed successfully")
            } else if !phone_book.contains_key(&name) {
                println!("The file dosen't contain the data");
            }
        } else {
            println!("try again")
        }
    }
}

fn map_writer(
    phone_book: BTreeMap<String, String>,
    file_path: String,
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

fn map_reader(file_path1: String) -> Result<BTreeMap<String, String>, Box<dyn std::error::Error>> {
        if !std::path::PathBuf::from(&file_path1).exists() {
            return Ok(BTreeMap::new());
        }

        let r = read(file_path1)?;
        let x = String::from_utf8(r)?;
        let x_split: Vec<&str> = x.split("\n").collect();
        let mut map = BTreeMap::new();
        for word in x_split {
            let word_split: Vec<&str> = word.split(": ").collect();
            map.insert(word_split[0].to_string(), word_split[1].to_string());
   }
   Ok(map)    
}


