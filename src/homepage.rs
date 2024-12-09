extern crate python_input;
use crate::clear_console;
use crate::friends_cli::friends::friends;
use crate::group_chats_cli::group_chats::group_chats;
use mongodb::Database;
use python_input::input;

pub async fn homepage(database: Database, user_email: String) {
    clear_console();
    println!("\x1b[1mHomepage\x1b[0m");
    println!();
    println!("[1] Friends List");
    println!("[2] Group Chats");
    println!();

    let choice = input("Which would you like to access? ");

    match choice.as_str() {
        "1" => {
            // clear_console();
            friends(database, user_email).await;
        }
        "2" => {
            // clear_console();
            group_chats(database, user_email).await;
        }
        _ => {
            println!("Invalid option, please try again.");
        }
    }
}
