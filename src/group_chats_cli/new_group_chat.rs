extern crate python_input;
use std::sync::{Arc, Mutex};

use super::group_chats_routes::{add_group_chat_w_db, AddGroupChatOutcome};
use crate::{clear_console, group_chats_cli::group_chat_messages::group_chat_messages};
use mongodb::Database;
use python_input::input;

/// Function to create a new group chat and open its messages view upon success.
pub async fn new_group_chat(database: Database, user_email: String) {
    clear_console();
    println!("\x1b[1mCreate New Group Chat\x1b[0m");
    println!();

    let complete_status = Arc::new(Mutex::new(false));
    let complete_status_clone = Arc::clone(&complete_status);

    loop {
        // Provide options to the user
        println!("\nOptions:");
        println!("[back] Back to Homepage\n");

        // Get the group chat name
        let group_chat_name = loop {
            let input_name = input("Chat name: ").trim().to_string();
            if input_name.eq_ignore_ascii_case("back") {
                // clear_console();
                let mut curr_completion_status = complete_status_clone.lock().unwrap();
                *curr_completion_status = true;
                return;
            }
            if input_name.is_empty() {
                println!("Error: Group chat name cannot be empty. Please try again.");
            } else {
                break input_name;
            }
        };

        // Get the list of friends to add
        let friends_emails = loop {
            println!("\nEnter the email addresses of friends to add, separated by commas:");
            let friends_input = input("Friends: ").trim().to_string();
            if friends_input.eq_ignore_ascii_case("back") {
                // clear_console();
                let mut curr_completion_status = complete_status_clone.lock().unwrap();
                *curr_completion_status = true;
                return;
            }

            let emails: Vec<String> = friends_input
                .split(',')
                .map(|email| email.trim().to_string())
                .filter(|email| !email.is_empty())
                .collect();

            if emails.is_empty() {
                println!("Error: Please provide at least one friend's email address.");
            } else {
                break emails;
            }
        };

        // Attempt to create the group chat
        match add_group_chat_w_db(
            database.clone(),
            group_chat_name.clone(),
            user_email.clone(),
            friends_emails.clone(),
        )
        .await
        {
            Ok(AddGroupChatOutcome::Success(group_chat_id)) => {
                println!("Group chat '{}' created successfully!", group_chat_name);

                // Open the group chat messages
                match group_chat_messages(
                    database.clone(),
                    user_email.clone(),
                    group_chat_id,
                    group_chat_name.clone(),
                )
                .await
                {
                    Ok(_) => {
                        println!("Exiting group chat...");
                        return;
                    }
                    Err(err) => {
                        println!(
                            "Error: Could not open the group chat '{}'. {}",
                            group_chat_name, err
                        );
                        return;
                    }
                }
            }
            Ok(AddGroupChatOutcome::CreatorEmailNotFound) => {
                println!("Error: Your email '{}' was not found.", user_email);
                return;
            }
            Ok(AddGroupChatOutcome::SomeEmailNotFound(email)) => {
                println!("Error: Friend's email '{}' was not found.", email);
            }
            Ok(AddGroupChatOutcome::SomeEmailNotFriends(email)) => {
                println!("Error: You are not friends with '{}'.", email);
            }
            Err(error) => {
                println!("Error: Failed to create group chat. {}", error);
            }
        }
    }
}
