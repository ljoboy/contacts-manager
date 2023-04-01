use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

const CONTACTS_FILE: &str = "contacts.json";

#[derive(Serialize, Deserialize)]
pub struct Contact {
    phone: String,
    name: String,
    address: String,
}

impl Contact {
    pub fn new(phone: String, name: String, address: Option<String>) -> Contact {
        Contact {
            phone,
            name,
            address: address.unwrap_or("".to_string()),
        }
    }

    pub fn get_phone(&self) -> &String {
        &self.phone
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_address(&self) -> &String {
        &self.address
    }
}

impl Display for Contact {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Phone: {}, Name: {}, Address: {}", self.phone, self.name, self.address)
    }
}

// serialize a vector of contacts to a file
pub fn serialize_contacts(contacts: &Vec<Contact>) -> std::io::Result<()> {
    let mut file = contacts_file();
    let serialized = serde_json::to_string(&contacts)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

// deserialize a vector of contacts from a file
pub fn deserialize_contacts() -> std::io::Result<Vec<Contact>> {
    let mut file = contacts_file();
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    if contents.is_empty() {
        return Ok(Vec::new());
    }

    let contacts: Vec<Contact> = serde_json::from_str(&contents)?;
    Ok(contacts)
}

fn contacts_file() -> File {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(CONTACTS_FILE)
        .expect("Failed to open contacts file");

    file
}