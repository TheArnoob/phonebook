use std::{collections::BTreeMap, fs::read, io::Write};

const FILE_NAME: &str = "file2.txt";

fn main() {
    loop {
        println!("Enter 1 of these commands:");
        println!("show, exit, add, remove");
        let mut string = String::new();
        std::io::stdin().read_line(&mut string).unwrap();
        let parsed_string = string.trim().parse::<String>().unwrap();
        if parsed_string == "show" {
            let result = map_reader(FILE_NAME.to_string()).expect("Cannot read data");
            println!("{result:?}");
        } else if parsed_string == "exit" {
            return;
        } else if parsed_string == "add" {
            println!("please enter a name");
            let mut string1 = String::new();
            std::io::stdin().read_line(&mut string1).unwrap();
            println!("please enter a number");
            let mut string2 = String::new();
            std::io::stdin().read_line(&mut string2).unwrap();
            let mut result = map_reader(FILE_NAME.to_string()).expect("Cannot read data");
            result.insert(string1.trim().to_string(), string2.trim().to_string());
            map_writer(result, FILE_NAME.to_string()).expect("failed to write");
        } else if parsed_string == "remove" {
            println!("Please enter a name");
            let mut string3: String = String::new();
            std::io::stdin().read_line(&mut string3).unwrap();
            let string3 = string3.trim().to_string();
            let mut result = map_reader(FILE_NAME.to_string()).unwrap();
            if result.contains_key(&string3) {
                result.remove(&string3);
                map_writer(result, FILE_NAME.to_string()).unwrap();
                println!("Entry removed successfully")
            } else if !result.contains_key(&string3) {
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

fn map_reader(file_path: String) -> Result<BTreeMap<String, String>, Box<dyn std::error::Error>> {
    let r = read(file_path)?;
    let x = String::from_utf8(r)?;
    let x_split: Vec<&str> = x.split("\n").collect();

    let mut strings = BTreeMap::new();
    for word in x_split {
        let word_split: Vec<&str> = word.split(": ").collect();
        strings.insert(word_split[0].to_string(), word_split[1].to_string());
    }
    Ok(strings)
}
