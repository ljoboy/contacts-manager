use std::fmt::{Display, Formatter};

struct Contact {
    phone: String,
    first_name: String,
    last_name: String,
}

impl Contact {
    fn new(phone: String, first_name: String, last_name: String) -> Contact {
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

fn main() {
    let contact = Contact::new("123456789".to_string(), "John".to_string(), "Doe".to_string());
    println!("{}", contact);
}