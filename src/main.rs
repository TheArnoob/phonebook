use std::{collections::BTreeMap, fs::read, io::Write};

fn main() {
    // let mut phone_book = BTreeMap::new();
    // phone_book.insert("john".to_string(), "0505555555".to_string());
    // phone_book.insert("mark".to_string(), "0506666666".to_string());
    // phone_book.insert("jack".to_string(), "0507777777".to_string());


    // map_writer(phone_book, "file2.txt".to_string()).expect("failed to write");

    loop {
        println!("Enter one of these commands:");
        println!("show, exit");
        let mut string = String::new();
        std::io::stdin().read_line(&mut string).unwrap();
        let parsed_string = string.trim().parse::<String>().unwrap();
        if parsed_string == "show" {
            let result = map_reader("file2.txt".to_string()).expect("Cannot read data");
            println!("{result:?}");
        } else if parsed_string == "exit" {
            return;
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
    let r: Vec<u8> = read(file_path)?;
    let x = String::from_utf8(r)?;
    let x_split: Vec<&str> = x.split("\n").collect();

    let mut strings = BTreeMap::new();
    for word in x_split {
        let word_split: Vec<&str> = word.split(": ").collect();
        strings.insert(word_split[0].to_string(), word_split[1].to_string());
    }
    Ok(strings)
}
