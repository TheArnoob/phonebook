use std::{collections::BTreeMap, io::Write};

fn main() {
    let mut phone_book = BTreeMap::new();
    phone_book.insert("john", "0505555555");
    phone_book.insert("mark", "0506666666");
    phone_book.insert("jack", "0507777777");


    map_writer(phone_book, "file2.txt".to_string()).expect("failed to write");
}

fn map_writer(
    phone_book: BTreeMap<&str, &str>,
    file_path: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = std::fs::File::create(file_path)?;
    for (name, phone_number) in phone_book {
        file.write_all(name.as_bytes())?;
        file.write_all(": ".as_bytes())?;
        file.write_all(phone_number.as_bytes())?;
        file.write_all("\n".as_bytes())?;
    }
    Ok(())
}
