extern crate python_input;
use python_input::input;

use email_address::*;

use ::regex::Regex;

mod authentication;


pub async fn login_signup_cli() -> Option<String> {
  let MONGODB_URI = "mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0".to_string();
  println!("\x1b[1mWelcome to CLIChat!\x1b[0m\n");
  println!("[1] Login");
  println!("[2] Sign Up");
  println!("\n\n\n\n\n\n\n\n\n\n");
  loop {
      // Get input from the user
      let input_value = input("> ");
      let mut email_input: String;
      let mut password_input: String;
      if input_value.trim() == "1" || input_value.trim() == "2"   {
        loop {
          loop {
            email_input = input("What is your email?: ");
            // validate email input
            let email_regex = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();

            if !email_regex.is_match(&email_input) {
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
          if input_value.trim() == "1" {
            match authentication::sign_in(
              MONGODB_URI.clone(), email_input.clone(), password_input
            ).await {
              Ok(authentication::SignInOutcome::Success) => {
                println!("Welcome to CLIChat!");
                return Some(email_input.clone());
              },
              Ok(authentication::SignInOutcome::EmailNotFound) => {
                println!("Email not found, please try again.");
              },
              Ok(authentication::SignInOutcome::IncorrectPassword) => {
                println!("Incorrect password, please try again.");
              },
              Err(e) => {
                println!("{}", e);
              }
            }
          } else if input_value.trim() == "2" {
            match authentication::register_account(
              MONGODB_URI.clone(), email_input.clone(), password_input
            ).await {
              Ok(success) => {
                if success {
                  println!("Welcome to CLIChat!");
                  return Some(email_input.clone());
                } else {
                  println!("User already exists. Please use another email address.");
                }
              },
              Err(e) => {
                println!("{}", e);
              }
            }
          } else {
            println!("{} is not a valid option.", input_value);
          }
        }
      }
  }
}
