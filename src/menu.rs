use crate::repertoire::{Contact, deserialize_contacts, serialize_contacts};

// menu function to display the menu and get the user's choice
fn menu() {
    loop {
        println!("Menu : 1. Add a contact, 2. Display all contacts, 3. Search a contact, 4. Delete a contact, 5. Quit");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u8 = choice.trim().parse().expect("Please type a number!");

        match choice {
            1 => add_contact(),
            2 => display_contacts(),
            3 => search_contact(),
            4 => delete_contact(),
            5 => {
                println!("Goodbye!");
                break;
            },
            _ => println!("Please choose a number between 1 and 5"),
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
    }
}