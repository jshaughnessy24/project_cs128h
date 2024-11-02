extern crate python_input;
use python_input::input;

use email_address::*;
mod login_and_signup;
mod authentication;


pub async fn login_signup_cli() {
  let MONGODB_URI = "mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0".to_string();
  println!("Welcome to CLIChat!\n");
  println!("[1] Login");
  println!("[2] Sign Up");
  loop {
      // Get input from the user
      let input_value = input("> ");
      let mut email_input: String = String::new();
      let mut password_input: String = String::new();
      if input_value.trim() == "1" || input_value.trim() == "2"   {
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
      }
      if input_value.trim() == "1" {
        match authentication::sign_in(
          MONGODB_URI, email_input, password_input
        ).await {
          Ok(authentication::SignInOutcome::Success) => {
            println!("Welcome to CLIChat!");
            break;
          },
          Ok(authentication::SignInOutcome::UsernameNotFound) => {
            println!("Username not found, please try again.");
          },
          Ok(authentication::SignInOutcome::IncorrectPassword) => {
            println!("Incorrect password, please try again.");
          },
          Err(e) => {
            println!("{}", e);
          }
        }
        break;
      } else if input_value.trim() == "2" {
        match authentication::register_account(
          MONGODB_URI, email_input, password_input
        ).await {
          Ok(_) => {
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
