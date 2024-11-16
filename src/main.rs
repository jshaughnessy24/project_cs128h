mod friends_cli;
use friends_cli::friends::friends;
mod login_signup_cli;
use std::process::{Command, Stdio};

use mongodb::Client;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let client: Result<Client, mongodb::error::Error> = Client::with_uri_str("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0").await;
    let database = client.unwrap().database("cli_chat");

    print!("{}[2J", 27 as char);
    let user_email = login_signup_cli::login_signup_cli().await;
    print!("{}[2J", 27 as char);

    friends(database, user_email.unwrap()).await;
    // match current_user_email {
    //     Some(user_email) => {
    //         message_cli::message_cli(user_email.to_string(), "test@test.com".to_string()).await;
    //     },
    //     _ => {
    //         println!("No signed in user!");
    //     }
    // }

    Ok(())
}
