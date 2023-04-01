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
}
