use prettytable::{format, row, Table};
use crate::repertoire::{Contact, deserialize_contacts, empty_contacts, serialize_contacts};

// menu function to display the menu and get the user's choice
pub fn menu() {
    loop {
        println!("Menu : 1. Add a contact, 2. Display all contacts, 3. Search a contact, 4. Delete a contact, 0. Quit");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u8 = choice.trim().parse().expect("Please type a number!");

        match choice {
            1 => add_contact(),
            2 => display_contacts(),
            3 => search_contact(),
            4 => delete_contact(),
            0 => {
                println!("Goodbye!");
                break;
            }
            _ => eprintln!("Please choose a number within the menu!"),
        }
    }

    // add a contact
    fn add_contact() {
        println!("Enter the phone number:");
        let mut phone = String::new();
        std::io::stdin().read_line(&mut phone).expect("Failed to read line");
        let phone = phone.trim().to_string();

        println!("Enter the name:");
        let mut first_name = String::new();
        std::io::stdin().read_line(&mut first_name).expect("Failed to read line");
        let first_name = first_name.trim().to_string();

        let contact = Contact::new(phone, first_name, None);
        let mut contacts = deserialize_contacts().unwrap();
        contacts.push(contact);
        serialize_contacts(&contacts).unwrap();
        println!("Contact added successfully!");
    }

    // display all contacts
    fn display_contacts() {
        let contacts = deserialize_contacts().unwrap();
        display_table(contacts);
    }

    // search a contact
    fn search_contact() {
        println!("Enter the search term:");
        let mut search_term = String::new();
        std::io::stdin().read_line(&mut search_term).expect("Failed to read line");
        let search_term = search_term.trim().to_string();

        let contacts = deserialize_contacts().unwrap();
        display_table(
            contacts.into_iter()
                .filter(|contact|
                    contact.get_phone().contains(&search_term) ||
                        contact.get_name().contains(&search_term)
                )
                .collect()
        );
    }

    // delete a contact
    fn delete_contact() {
        let mut contacts = deserialize_contacts().unwrap();
        let mut phone = String::new();
        println!("Enter the phone number of the contact to delete: ");
        std::io::stdin().read_line(&mut phone).expect("Failed to read line");

        let phone = phone.trim().to_string();

        if let Some(index) = contacts.iter().position(|contact| contact.get_phone() == &phone) {
            println!("Are you sure? {}", contacts.get(index).unwrap());
            println!("y/n default: n");
            let mut answer = String::new();
            std::io::stdin().read_line(&mut answer).expect("Failed to read line");
            let answer = answer.trim().to_string().to_lowercase();
            if answer != "y" && answer != "yes" {
                println!("Contact not deleted.");2
                return;
            }

            contacts.remove(index);
            empty_contacts();
            serialize_contacts(&contacts)
                .unwrap_or_else(|_| eprintln!("Failed to write contacts file."));
            println!("Contact deleted successfully!");
        } else {
            eprintln!("Contact not found.");
        }
    }

    fn display_table(contacts: Vec<Contact>) {
        let mut table = Table::new();
        table.set_titles(row!["Phone", "Name", "Address"]);
        for contact in contacts {
            table.add_row(row![contact.get_phone(), contact.get_name(), contact.get_address()]);
        }
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        table.printstd();
    }
}
