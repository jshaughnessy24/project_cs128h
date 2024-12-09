use std::cmp::max;
use std::cmp::min;
use std::thread;
extern crate python_input;

use super::group_chats_routes::{get_messages_group_chat, send_message_group_chat_w_db, Message};
use chrono;
use futures::StreamExt;
use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use python_input::input;

use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use tokio::runtime::Runtime;
use tokio::time::*;

use mongodb::{
    bson::{self, doc, Bson, Document},
    Client, Collection, Database,
};

// async function to listen for changes to messages
async fn listen_for_changes(
    database: Database,
    messages: Arc<Mutex<Vec<Message>>>,
    current_user_email: String,
    group_chat_id: ObjectId,
    group_chat_name: String,
    shared_counter: Arc<Mutex<usize>>,
) -> mongodb::error::Result<()> {
    let messages_coll: Collection<Document> = database.clone().collection("messages");
    let mut change_stream = messages_coll.watch().await?;
    while let Some(event) = change_stream.next().await.transpose()? {
        if let Some(doc) = event.full_document {
            let recipient_email = doc
                .get("recipient_email")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string();
            if recipient_email == current_user_email {
                let new_message = Message {
                    sender: doc
                        .get("author_email")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string(),
                    date_string: doc
                        .get("date_sent")
                        .unwrap()
                        .as_datetime()
                        .unwrap()
                        .to_string(),
                    content: doc
                        .get("message_content")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string(),
                };
                let mut msgs = messages.lock().unwrap();
                msgs.push(new_message);
                let mut start = shared_counter.lock().unwrap();

                if msgs.len() > 3 {
                    *start = msgs.len() - 3;
                } else {
                    *start = 0;
                }
                print_messages(
                    &msgs,
                    current_user_email.clone(),
                    group_chat_id,
                    group_chat_name.clone(),
                    start.clone(),
                );
                println!("");
                println!("Submit your message, or navigate by typing up or down: ");
            }
        };
    }
    Ok(())
}

// main function for handling the group chat messages
pub async fn group_chat_messages(
    database: Database,
    current_user_email: String,
    group_chat_id: ObjectId,
    group_chat_name: String,
) -> mongodb::error::Result<()> {
    let mut messages_list: Vec<Message> = Vec::new();

    let all_messages = get_messages_group_chat(database.clone(), group_chat_id.clone()).await;

    match all_messages {
        Ok(Some(messages)) => {
            messages_list = messages;
        }
        Err(err) => {
            println!("An error occurred: {}", err);
            return Ok(());
        }
        _ => {
            println!("An error occurred, please try again.");
            return Ok(());
        }
    }

    let messages = Arc::new(Mutex::new(messages_list.clone()));
    let messages_for_input = Arc::clone(&messages);

    let mut start = 0;

    if messages_list.len() > 3 {
        start = messages_list.len() - 3;
    }

    let shared_counter = Arc::new(Mutex::new(start));

    let shared_counter1: Arc<Mutex<usize>> = Arc::clone(&shared_counter);
    let shared_counter2: Arc<Mutex<usize>> = Arc::clone(&shared_counter);

    let current_user_email_arc = Arc::new(current_user_email.clone());
    let group_chat_id_arc = Arc::new(group_chat_id.clone());

    let current_user_email_input: Arc<String> = Arc::clone(&current_user_email_arc);
    let current_user_email_input2: Arc<String> = Arc::clone(&current_user_email_arc);
    let group_chat_id_input: Arc<ObjectId> = Arc::clone(&group_chat_id_arc);

    let database_arc = Arc::new(Mutex::new(database.clone()));
    let database_clone = Arc::clone(&database_arc);

    let complete_status = Arc::new(Mutex::new(false));

    let complete_status1: Arc<Mutex<bool>> = Arc::clone(&complete_status);

    let group_chat_name_clone1 = group_chat_name.clone();

    // spawn a task to listen to user input
    tokio::spawn(async move {
        loop {
            let mut input = String::new();
            print!("Submit your message, or navigate by typing up or down: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let message_input = input.trim().to_string();
            if !input.is_empty() {
                if message_input == "up" {
                    let mut start = shared_counter.lock().unwrap();
                    if (*start > 2) {
                        *start = *start - 1;
                    }
                } else if message_input == "down" {
                    let mut start = shared_counter.lock().unwrap();
                    let mut msgs = messages_for_input.lock().unwrap();
                    if (*start < msgs.len() - 3) {
                        *start = *start + 1;
                    }
                } else if message_input == "back" {
                    let mut completion_status = complete_status1.lock().unwrap();
                    *completion_status = true;
                    break;
                } else {
                    send_message_group_chat_w_db(
                        database.clone(),
                        current_user_email_input.to_string(),
                        group_chat_id,
                        message_input.clone(),
                    )
                    .await;
                    let mut msgs = messages_for_input.lock().unwrap();
                    msgs.push(Message {
                        sender: current_user_email_input.to_string(),
                        date_string: format!("{:?}", chrono::offset::Local::now()),
                        content: message_input.clone(),
                    });

                    let mut start = shared_counter.lock().unwrap();
                    if msgs.len() > 3 {
                        *start = msgs.len() - 3;
                    } else {
                        *start = 0;
                    }
                }
                let mut msgs = messages_for_input.lock().unwrap();
                let mut start = shared_counter1.lock().unwrap();
                print_messages(
                    &msgs,
                    group_chat_id_input.to_string(),
                    group_chat_id,
                    group_chat_name_clone1.clone(),
                    start.clone(),
                );
            }
        }
    });

    // Spawn a task to listen for database changes
    tokio::spawn(async move {
        let db = {
            let db_lock = database_clone.lock().unwrap();
            db_lock.clone()
        };

        listen_for_changes(
            db,
            messages.clone(),
            current_user_email_input2.to_string(),
            group_chat_id.clone(),
            group_chat_name.clone(),
            shared_counter2.clone(),
        )
        .await;
    });

    // Wait for the user to exit the input loop
    loop {
        let completion_status = complete_status.lock().unwrap();
        if *completion_status {
            break;
        }
        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}

// Function to print messages in the group chat
fn print_messages(
    messages_list: &Vec<Message>,
    recipient_email: String,
    group_chat_id: ObjectId,
    group_chat_name: String,
    start: usize,
) {
    println!("{}[2J", 27 as char);
    println!("\x1b[1m{}\x1b[0m\n", group_chat_name);
    println!("\n[back] Back to group chats menu\n");
    for i in max(0, start)..min(messages_list.len(), start + 3) {
        println!(
            "[{}, {}]",
            messages_list[i].sender, messages_list[i].date_string
        );
        println!("{}\n", messages_list[i].content);
    }
}
