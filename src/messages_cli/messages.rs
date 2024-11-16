
use std::cmp::max;
use std::cmp::min;
extern crate python_input;
use python_input::input;
use super::messages_routes::{Message, get_messages, send_message_w_db};
use chrono;

use mongodb::{Client, Database};

pub async fn messages(
    database: Database,
    current_user_email: String,
    recipient_email: String
) {

    let mut messages_list: Vec<Message> = Vec::new();

    let all_messages = get_messages(
        database.clone(),
        current_user_email.clone(),
        recipient_email.clone()
    ).await;

    match all_messages {
        Ok(Some(messages)) => {
            messages_list = messages;
        }
        Err(err) => {
            println!("An error occurred: {}", err);
            return;
        }
        _ => {
            println!("An error occurred, please try again.");
            return;
        }
    }

    let mut start = 0;
    if messages_list.len() > 3 {
        start = messages_list.len() - 3;
    }

    loop {
        println!("\x1b[1mDirect Messages with {}\x1b[0m\n", recipient_email);
        println!("\n[back] Back to friends list");
        for i in max(0,start)..min(messages_list.len(), start+3) {
            println!("[{}, {}]", messages_list[i].sender, messages_list[i].date_string);
            println!("{}\n", messages_list[i].content);
        }
        println!("Submit your message, or navigate by typing up or down.");
        let message_input = input("> "); 
        if message_input == "up".to_string() {
            if (start > 2) { // prevent from going above the top
                start = start - 1;
            }
        } else if message_input == "down".to_string() {
            if (start < messages_list.len() - 3) { // prevent from going below the bottom
                start = start + 1;
            }
        } else if message_input == "back".to_string() {
            break;
        } else {
            send_message_w_db(
                database.clone(),
                current_user_email.clone(),
                recipient_email.clone(),
                message_input.clone()
            ).await;

            let all_messages2 = get_messages(
                database.clone(),
                current_user_email.clone(),
                recipient_email.clone()
            ).await;
            
            match all_messages2 {
                Ok(Some(messages)) => {
                    messages_list = messages;
                }
                Err(err) => {
                    println!("An error occurred: {}", err);
                    return;
                }
                _ => {
                    println!("An error occurred, please try again.");
                    return;
                }
            }


            messages_list.push(Message {
                sender: current_user_email.to_string(),
                date_string: "11/9/2024 11:50pm".to_string(),
                content: message_input
            });
            
            if messages_list.len() > 3 {
                start = start + 1;
            } else {
                start = 0;
            }
        }
        print!("{}[2J", 27 as char);

    }
}