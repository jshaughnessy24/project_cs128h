extern crate python_input;
use python_input::input;

use mongodb::{Client, Database};

use crate::friends_cli::friends_routes::{add_friend_w_db, get_friend_list, AddFriendOutcome};

pub async fn friends(database: Database, current_username: String) {
    let mut friend_list: Vec<String> = Vec::new();

    loop {
        println!("\x1b[1mFriends List\x1b[0m\n");

        let username = current_username.to_string();

        match get_friend_list(database.clone(), username.clone()).await {
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
                println!("User '{}' not found.", username);
            }
            Err(err) => {
                println!("Error retrieving friend list: {}", err);
            }
        }

        println!("\nOptions:");
        println!("[back] Back to Homepage");
        println!("[add-friend] [name] Add a friend");
        println!("[remove-friend] [name] Remove a friend");
        println!("[direct-message] [name] DM friend\n");

        let choice = input("What would you like to do? ");

        let parts: Vec<&str> = choice.split_whitespace().collect();
        match parts.as_slice() {
            ["back"] => {
                println!("Returning to Homepage...");
                break;
            }
            ["add-friend", friend_username] => {
                match add_friend_w_db(
                    database.clone(),
                    username.clone(),
                    friend_username.to_string(),
                )
                .await
                {
                    Ok(AddFriendOutcome::Success) => {
                        println!("Friend '{}' added!", friend_username);
                    }
                    Ok(AddFriendOutcome::CurrentUsernameNotFound) => {
                        println!("Your username was not found.");
                    }
                    Ok(AddFriendOutcome::OtherUsernameNotFound) => {
                        println!("Friend '{}' not found.", friend_username);
                    }
                    Ok(AddFriendOutcome::AlreadyFriends) => {
                        println!("You are already friends with '{}'.", friend_username);
                    }
                    Err(err) => {
                        println!("Failed to add friend: {}", err);
                    }
                }
            }
            ["remove-friend", friend_username] => {
                // TODO
                println!("need to implement");
            }
            ["direct-message", friend_username] => {
                if friend_list.contains(&friend_username.to_string()) {
                    println!("Direct messaging {}...", friend_username);
                    // TODO
                } else {
                    println!(
                        "Friend '{}' not found in your friend list.",
                        friend_username
                    );
                }
            }
            _ => {
                println!("Invalid option, please try again.");
            }
        }
    }
}
