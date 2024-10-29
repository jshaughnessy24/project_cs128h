extern crate python_input;
use python_input::input;
use email_address::*;
mod login_and_signup;
pub fn login_signup_cli() {
    println!("Welcome to CLIChat!\n");
    println!("[1] Login");
    println!("[2] Sign Up");
    loop {
        // Get input from the user
        let input_value = input("> ");
        let mut email_input: String = String::new();
        let mut password_input: String = String::new();
        if input_value.trim() == "1" {
          loop {
            email_input = input("What is your email?: ");
            // validate email input
            if !EmailAddress::is_valid(&email_input) {
              println!("Invalid email, please try again.");
            } else {
              break;
            }
          }
          loop {
            password_input = input("What is your password?: ");
            if password_input.trim() == "" {
              println!("Invalid password, please try again.");
            } else {
              break;
            }
          }
          match login_and_signup::login(
            email_input, password_input
          ) {
            Ok(()) => {
              println!("Welcome to CLIChat!");
              break;
            },
            Err(e) => {
              println!("{}", e);
            }
          }
          break;
        } else {
          println!("{} is not a valid option.", input_value);
        }
    }
}
