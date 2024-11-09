extern crate python_input;
use python_input::input;

use mongodb::{Client, Database};

use crate::friends_cli::friends_routes::{add_friend_w_db, get_friend_list, AddFriendOutcome};

pub async fn friends(database: Database, current_email: String) {
    let mut friend_list: Vec<String> = Vec::new();

    loop {
        println!("\x1b[1mFriends List\x1b[0m\n");

        let email = current_email.to_string();

        match get_friend_list(database.clone(), email.clone()).await {
            Ok(Some(friends)) => {
                if friends.is_empty() {
                    println!("You have no friends added.");
                } else {
                    for (i, friend) in friends.iter().enumerate() {
                        println!("{}: {}", i + 1, friend);
                    }
                }
                friend_list = friends;
            }
            Ok(None) => {
                println!("User '{}' not found.", email);
            }
            Err(err) => {
                println!("Error retrieving friend list: {}", err);
            }
        }

        println!("\nOptions:");
        println!("[back] Back to Homepage");
        println!("[add-friend] [email] Add a friend");
        println!("[remove-friend] [email] Remove a friend");
        println!("[direct-message] [email] DM friend\n");

        let choice = input("What would you like to do? ");

        let parts: Vec<&str> = choice.split_whitespace().collect();
        match parts.as_slice() {
            ["back"] => {
                println!("Returning to Homepage...");
                break;
            }
            ["add-friend", friend_email] => {
                match add_friend_w_db(database.clone(), email.clone(), friend_email.to_string())
                    .await
                {
                    Ok(AddFriendOutcome::Success) => {
                        println!("Friend '{}' added!", friend_email);
                    }
                    Ok(AddFriendOutcome::CurrentEmailNotFound) => {
                        println!("Your email was not found.");
                    }
                    Ok(AddFriendOutcome::OtherEmailNotFound) => {
                        println!("Friend '{}' not found.", friend_email);
                    }
                    Ok(AddFriendOutcome::AlreadyFriends) => {
                        println!("You are already friends with '{}'.", friend_email);
                    }
                    Err(err) => {
                        println!("Failed to add friend: {}", err);
                    }
                }
            }
            ["remove-friend", friend_email] => {
                // TODO
                println!("need to implement");
            }
            ["direct-message", friend_email] => {
                if friend_list.contains(&friend_email.to_string()) {
                    println!("Direct messaging {}...", friend_email);
                    // TODO
                } else {
                    println!("Friend '{}' not found in your friend list.", friend_email);
                }
            }
            _ => {
                println!("Invalid option, please try again.");
            }
        }
    }
}
