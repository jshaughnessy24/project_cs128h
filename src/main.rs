mod friends_cli;
use friends_cli::friends::friends;
mod homepage;
use homepage::homepage;
mod group_chats_cli;
mod login_signup_cli;
use group_chats_cli::group_chats::group_chats;

mod messages_cli;

mod clear_console;
use clear_console::clear_console::clear_console;

use mongodb::Client;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    // Connect to the MongoDB client and get the database
    let client: Result<Client, mongodb::error::Error> = Client::with_uri_str("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0").await;
    let database = client.unwrap().database("cli_chat");

    clear_console();
    let user_email = login_signup_cli::login_signup_cli().await;
    clear_console();

    homepage(database, user_email.unwrap()).await;

    // group_chats(database, user_email.unwrap()).await;

    // homepage(database, user_email.unwrap()).await;

    // friends(database, user_email.unwrap()).await;

    Ok(())
}
