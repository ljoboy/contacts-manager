mod repertoire;


fn main() -> std::io::Result<()> {
    let contact = repertoire::Contact::new("1234567890".to_string(), "John".to_string(), "Doe".to_string());
    println!("{}", contact);
    Ok(())
}