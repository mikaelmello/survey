use inquire::{Password, PasswordDisplayMode};

fn main() {
    let name = Password::new("RSA Encryption Key:")
        .with_display_toggle_enabled()
        .with_display_mode(PasswordDisplayMode::UnmaskedLastChar)
        .prompt();

    match name {
        Ok(_) => println!("This doesn't look like a key."),
        Err(_) => println!("An error happened when asking for your key, try again later."),
    }
}
