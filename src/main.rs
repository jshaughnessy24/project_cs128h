mod friends_cli; // Declare the friends_cli module directory
use friends_cli::friends::friends; // Import the friends function from the friends module
mod login_signup_cli;
mod message_cli;
use std::process::{Command, Stdio};

use mongodb::Client;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let client: Result<Client, mongodb::error::Error> = Client::with_uri_str("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0").await;
    let database = client.unwrap().database("cli_chat");

    print!("{}[2J", 27 as char);
    let email_input = login_signup_cli::login_signup_cli().await;
    print!("{}[2J", 27 as char);

    friends(database, email_input.unwrap()).await; // Call the friends function correctly
    Ok(())
}
