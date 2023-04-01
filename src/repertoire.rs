use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Write, Read};

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
    let mut file = File::create(CONTACTS_FILE)?;
    let serialized = serde_json::to_string(&contacts)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

// deserialize a vector of contacts from a file
pub fn deserialize_contacts() -> std::io::Result<Vec<Contact>> {
    let mut file = File::open(CONTACTS_FILE)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let contacts: Vec<Contact> = serde_json::from_str(&contents)?;
    Ok(contacts)
}