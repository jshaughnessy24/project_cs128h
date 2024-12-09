extern crate python_input;
use super::group_chats_routes::{
    get_group_chat_ids_names_map, get_messages_group_chat, send_message_group_chat_w_db,
};
use crate::group_chats_cli::group_chat_messages::group_chat_messages;
use crate::group_chats_cli::new_group_chat::new_group_chat;
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use python_input::input;
use std::collections::HashMap;

pub async fn group_chats(database: Database, user_email: String) {
    println!("\x1b[1mGroup Chats\x1b[0m");
    println!();

    // Retrieve group chat names and IDs
    let group_chats_result =
        get_group_chat_ids_names_map(database.clone(), user_email.clone()).await;

    // Handle the Result returned from the function
    let group_chats: HashMap<ObjectId, String> = match group_chats_result {
        Ok(Some(chats)) => chats,
        Ok(None) => {
            println!("No group chats found.");
            return; // Early return if no chats found
        }
        Err(err) => {
            println!("Error retrieving group chats: {}", err);
            return; // Early return if there was an error
        }
    };

    println!("\nOptions:");
    println!("[back] Back to Homepage");
    println!("[new] Create New Group Chat");
    println!("\nSelect a chat:");

    // Display group chats in the format [1] Chat name
    if !group_chats.is_empty() {
        for (i, (_, chat_name)) in group_chats.iter().enumerate() {
            println!("[{}] {}", i + 1, chat_name);
        }
    } else {
        println!("You are not part of any group chats.");
    }

    // Get user input
    let choice = input("Which would you like to access? ");

    match choice.as_str() {
        "back" => {
            // Logic for returning to homepage or exiting
            println!("Returning to homepage...");
        }
        "new" => {
            // Logic for creating a new group chat
            new_group_chat(database, user_email).await;
        }
        _ => {
            if let Ok(index) = choice.parse::<usize>() {
                if index > 0 && index <= group_chats.len() {
                    let chat_id = group_chats.keys().nth(index - 1).unwrap();
                    let chat_name = group_chats.get(chat_id).unwrap().to_string();
                    println!("Accessing group chat: {}", chat_name);

                    // Call function to handle interaction with selected chat
                    access_group_chat(database, chat_id.clone(), chat_name, user_email).await;
                } else {
                    println!("Invalid chat selection.");
                }
            } else {
                println!("Invalid input. Please try again.");
            }
        }
    }
}

/// Handles interaction within a selected group chat
async fn access_group_chat(
    database: Database,
    group_chat_id: ObjectId,
    group_chat_name: String,
    user_email: String,
) {
    println!("\nAccessing group chat: {}\n", group_chat_name);

    group_chat_messages(
        database.clone(),
        user_email.clone(),
        group_chat_id,
        group_chat_name.clone(),
    )
    .await;
}
