use std::cmp::max;
use std::cmp::min;
use std::thread;

use crate::group_chats_cli::group_chats_routes::{
    get_messages_group_chat, send_message_group_chat_w_db, Message,
};
use chrono;
use futures::StreamExt;
use mongodb::bson::oid::ObjectId;

use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::clear_console::clear_console::clear_console;

use mongodb::{bson::Document, Collection, Database};

/// Listens for new incoming messages and prints them to console.
///   database: mongodb database
///   messages: list of messages
///   current_user_email: email of the current user
///   shared_start: start index to begin printing from
async fn listen_for_new_incoming_messages(
    database: Database,
    messages: Arc<Mutex<Vec<Message>>>,
    current_user_email: String,
    group_chat_name: String,
    shared_start: Arc<Mutex<usize>>,
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
                let mut start = shared_start.lock().unwrap();

                if msgs.len() > 3 {
                    *start = msgs.len() - 3;
                } else {
                    *start = 0;
                }
                print_messages(
                    &msgs,
                    // current_user_email.clone(),
                    group_chat_name.clone(),
                    start.clone(),
                );
                println!("");
                println!("Submit your message, or navigate by typing up or down: ");
            }
        };
    }
    return Ok(());
}

/// Prints received messages to console.
///   messages_list: list of messages to print
///   recipient_email: email of the recipient
///   group_chat_name: name of group chat
///   start: index to start printing from
fn print_messages(messages_list: &Vec<Message>, group_chat_name: String, start: usize) {
    clear_console();
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

/// Handles messages between users.
///   database: mongodb database
///   current_user_email: email of the current user
///   recipient_email: email of the recipient
pub async fn group_chat_messages(
    database: Database,
    current_user_email: String,
    // recipient_email: String,
    group_chat_id: ObjectId,
    group_chat_name: String,
) -> mongodb::error::Result<()> {
    clear_console();
    let mut messages_list: Vec<Message> = Vec::new();

    let all_messages: Result<Option<Vec<Message>>, _> =
        get_messages_group_chat(database.clone(), group_chat_id.clone()).await;

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

    let shared_start = Arc::new(Mutex::new(start));

    let shared_start1: Arc<Mutex<usize>> = Arc::clone(&shared_start);
    let shared_start2: Arc<Mutex<usize>> = Arc::clone(&shared_start);

    let current_user_email_arc = Arc::new(current_user_email.clone());
    let group_chat_id_arc = Arc::new(group_chat_id.clone());

    let current_user_email_input: Arc<String> = Arc::clone(&current_user_email_arc);
    let current_user_email_input2: Arc<String> = Arc::clone(&current_user_email_arc);
    let group_chat_id_input: Arc<ObjectId> = Arc::clone(&group_chat_id_arc);

    let database_arc = Arc::new(Mutex::new(database.clone()));
    let database_clone = Arc::clone(&database_arc);

    // When complete_status is true, the listener task will be aborted
    let complete_status = Arc::new(Mutex::new(false));

    let complete_status_clone = Arc::clone(&complete_status);

    tokio::spawn(async move {
        // This thread handles the initial loading of messages
        // and responds to user actions (send message, go up, down, back)
        let mut first_run = true;
        loop {
            if first_run {
                let mut msgs = messages_for_input.lock().unwrap();
                print_messages(&msgs, group_chat_name.clone(), start.clone());
                first_run = false;
            }
            println!("");
            let mut input = String::new();
            print!("Submit your message, or navigate by typing up or down: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let message_input = input.trim().to_string();
            if !input.is_empty() {
                if message_input == "up".to_string() {
                    // Move the start up 1
                    let mut start = shared_start.lock().unwrap();
                    if *start > 2 {
                        // Prevent from going above the top
                        *start = *start - 1;
                    }
                } else if message_input == "down".to_string() {
                    // Move the start down 1
                    let mut start = shared_start.lock().unwrap();
                    let mut msgs = messages_for_input.lock().unwrap();
                    if *start < msgs.len() - 3 {
                        // Prevent from going below the bottom
                        *start = *start + 1;
                    }
                } else if message_input == "back".to_string() {
                    // Return to the friends list. Break the loop.
                    clear_console();
                    let mut curr_completion_status = complete_status_clone.lock().unwrap();
                    *curr_completion_status = true;
                    break;
                } else {
                    // Send the message. The message is message_input.
                    send_message_group_chat_w_db(
                        database.clone(),
                        current_user_email_input.to_string(),
                        group_chat_id.clone(),
                        message_input.clone(),
                    )
                    .await;

                    // Add the new message.
                    let mut msgs = messages_for_input.lock().unwrap();
                    msgs.push(Message {
                        sender: current_user_email_input.to_string(),
                        date_string: format!("{:?}", chrono::offset::Local::now()),
                        content: message_input.clone(),
                    });

                    let mut start = shared_start.lock().unwrap();
                    if msgs.len() > 3 {
                        *start = msgs.len() - 3;
                    } else {
                        *start = 0;
                    }
                }
                let mut msgs = messages_for_input.lock().unwrap();
                let mut start = shared_start1.lock().unwrap();
                print_messages(&msgs, group_chat_name.clone(), start.clone());
            }
        }
    });

    let messages_for_receive = Arc::clone(&messages);

    let listener_task = tokio::spawn(async move {
        // This thread listens for new incoming messages
        let db = {
            let db_lock = database_clone.lock().unwrap();
            db_lock.clone()
        };
        listen_for_new_incoming_messages(
            db,
            messages_for_receive.clone(),
            current_user_email_input2.to_string(),
            group_chat_id_input.to_string(),
            shared_start2.clone(),
        )
        .await;
    });

    loop {
        // This keeps the main thread alive until the listener task is aborted
        if *complete_status.lock().unwrap() {
            listener_task.abort();
            break;
        }
        thread::sleep(Duration::from_secs(1));
    }

    return Ok(());
}
