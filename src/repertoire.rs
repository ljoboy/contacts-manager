use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Write, Read};

#[derive(Serialize, Deserialize)]
pub struct Contact {
    phone: String,
    first_name: String,
    last_name: String,
}

impl Contact {
    pub fn new(phone: String, first_name: String, last_name: String) -> Contact {
        Contact {
            phone,
            first_name,
            last_name,
        }
    }
}

impl Display for Contact {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Phone: {}, First Name: {}, Last Name: {}", self.phone, self.first_name, self.last_name)
    }
}

// serialize a vector of contacts to a file
pub fn serialize_contacts(contacts: &Vec<Contact>) -> std::io::Result<()> {
    let mut file = File::create("contacts.json")?;
    let serialized = serde_json::to_string(&contacts)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

// deserialize a vector of contacts from a file
pub fn deserialize_contacts() -> std::io::Result<Vec<Contact>> {
    let mut file = File::open("contacts.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let contacts: Vec<Contact> = serde_json::from_str(&contents)?;
    Ok(contacts)
}