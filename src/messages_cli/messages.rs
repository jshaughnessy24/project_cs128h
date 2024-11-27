
use std::cmp::max;
use std::cmp::min;
use std::thread;
extern crate python_input;
use python_input::input;
use super::messages_routes::{Message, get_messages, send_message_w_db};
use chrono;
use futures::TryStreamExt;
use futures::StreamExt;

use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::io::{self, Write};

use tokio::runtime::Runtime;
use tokio::time::*;

use mongodb::{
    bson::{self, doc, Bson, Document},
    Client, 
    Database, 
    Collection
};

// enum CustomEvent {
//     MongoDBChange(String)
// }

async fn listen_for_changes(
    database: Database,
    messages: Arc<Mutex<Vec<Message>>>,
    current_user_email: String,
    shared_counter: Arc<Mutex<usize>>
) -> mongodb::error::Result<()> {
    let messages_coll: Collection<Document> = database.clone().collection("messages");
    let mut change_stream = messages_coll.watch().await?;   
    // TODO: Filter this stream to incoming messages only


    // TODO: Update the list of messages sent

    while let Some(event) = change_stream.next().await.transpose()? { // this triggers a rerender
        // println!("Operation performed: {:?}", event.operation_type);
        // println!("Document: {:?}", event.full_document);
        if let Some(doc) = event.full_document {
            let recipient_email = doc.get("recipient_email").unwrap().as_str().unwrap().to_string();
            if recipient_email == current_user_email {
                let new_message = Message {
                    sender: doc.get("author_email").unwrap().as_str().unwrap().to_string(),
                    date_string: doc.get("date_sent").unwrap().as_datetime().unwrap().to_string(), 
                    content: doc.get("message_content").unwrap().as_str().unwrap().to_string()
                };
                let mut msgs = messages.lock().unwrap();
                msgs.push(new_message);
                let mut start = shared_counter.lock().unwrap();

                if msgs.len() > 3 {
                    *start = msgs.len() - 3;
                } else {
                    *start = 0;
                }
                print_messages(&msgs, current_user_email.clone(), start.clone());    
            }
        };



        // if let Some(document_data) = event.full_document {
        //     if let Some(recipient_email) = document_data["recipient_email"] {
        //     if recipient_email == current_user_email {
        //         let author_email = document_data["author_email"];
        //         let message_content = document_data["message_content"];
        //         let mut msgs = messages_for_input.lock().unwrap();
        //         msgs.push(Message {
        //             sender: current_user_email.clone(),
        //             date_string: format!("{:?}", chrono::offset::Local::now()),
        //             content: message_content.clone()
        //         });
        //         print_messages(&msgs, current_user_email.clone(), start.clone());           
        //     }
        // }
    
    }
    return Ok(());

}

pub async fn messages(
    database: Database,
    current_user_email: String,
    recipient_email: String
) -> mongodb::error::Result<()> {
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
    let recipient_email_arc = Arc::new(recipient_email.clone());

    let current_user_email_input: Arc<String> = Arc::clone(&current_user_email_arc);
    let current_user_email_input2: Arc<String> = Arc::clone(&current_user_email_arc);
    let recipient_email_input: Arc<String> = Arc::clone(&recipient_email_arc);

    let database_arc = Arc::new(Mutex::new(database.clone()));
    let database_clone = Arc::clone(&database_arc);

    // let complete_status = Arc::new(Mutex::new(false));

    // let complete_status1: Arc<Mutex<bool>> = Arc::clone(&complete_status);
    // let complete_status2: Arc<Mutex<bool>> = Arc::clone(&complete_status);

    tokio::spawn(async move {
        loop { // take user input
            let mut input = String::new();
            print!("Submit your message, or navigate by typing up or down: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let message_input = input.trim().to_string();
            if !input.is_empty() {
                if message_input == "up".to_string() {
                    let mut start = shared_counter.lock().unwrap();
                    if (*start > 2) { // prevent from going above the top
                        *start = *start - 1;
                    }
                } else if message_input == "down".to_string() {
                    let mut start = shared_counter.lock().unwrap();
                    let mut msgs = messages_for_input.lock().unwrap();
                    if (*start < msgs.len() - 3) { // prevent from going below the bottom
                        *start = *start + 1;
                    }
                } else if message_input == "back".to_string() {
                    // TODO: handle exit
                    // let mut completion_status = complete_status1.lock().unwrap();
                    // *completion_status = true;
                    break;
                } else {
                    send_message_w_db(
                        database.clone(),
                        current_user_email_input.to_string(),
                        recipient_email.to_string(),
                        message_input.clone()
                    ).await;
                    let mut msgs = messages_for_input.lock().unwrap();
                    msgs.push(Message {
                        sender: current_user_email_input.to_string(),
                        date_string: format!("{:?}", chrono::offset::Local::now()),
                        content: message_input.clone()
                    });

                    let mut start = shared_counter.lock().unwrap();
                    if msgs.len() > 3 {
                        *start = msgs.len() - 3;
                    } else {
                        *start = 0;
                    }   
                }
                // msgs.push(format!("You: {}", input));
                let mut msgs = messages_for_input.lock().unwrap();
                let mut start = shared_counter1.lock().unwrap();
                print_messages(&msgs, recipient_email_input.to_string(), start.clone());
            }
        }
    });
    let recipient_email_input2: Arc<String> = Arc::clone(&recipient_email_arc);

    let messages_for_receive = Arc::clone(&messages);

    tokio::spawn(async move {
        let db = {
            let db_lock = database_clone.lock().unwrap(); // Access the `Database`
            db_lock.clone() // Clone the `Database` for passing
        };
        listen_for_changes(
            db,
            messages_for_receive.clone(),
            current_user_email_input2.to_string(),
            shared_counter2.clone()
        ).await;
        // let messages_coll: Collection<Document> = database.clone().collection("messages");
        // let mut change_stream = messages_coll.watch().await?;
        // while let Some(event) = change_stream.next().await.transpose()? { // this triggers a rerender
        //     println!("Operation performed: {:?}", event.operation_type);
        //     println!("Document: {:?}", event.full_document);
        // }
        // let simulated_received_messages = vec![
        //     "Hello from Alice!",
        //     "Bob: How's it going?",
        //     "System: Server will restart soon."
        // ];
        // for msg in simulated_received_messages {
        //     thread::sleep(Duration::from_secs(5)); // Simulate a delay
        //     let mut msgs = messages_for_receive.lock().unwrap();
        //     msgs.push(Message {
        //         sender: current_user_email.to_string(),
        //         date_string: format!("{:?}", chrono::offset::Local::now()),
        //         content: msg.to_string()
        //     });
        //     print_messages(&msgs, recipient_email_input2.to_string(), start.clone());
        // }
    });

    loop {
        // let mut completion_status = complete_status2.lock().unwrap();
        // println!("Completion Status: {}", *completion_status);
        // if (*completion_status == true) {
        //     return Ok(());
        // }
        thread::sleep(Duration::from_secs(1));
    }

    return Ok(());



    // loop {
        // println!("\x1b[1mDirect Messages with {}\x1b[0m\n", recipient_email);
        // println!("\n[back] Back to friends list");
        // for i in max(0,start)..min(messages_list.len(), start+3) {
        //     println!("[{}, {}]", messages_list[i].sender, messages_list[i].date_string);
        //     println!("{}\n", messages_list[i].content);
        // }
    //     println!("Submit your message, or navigate by typing up or down.");
    //     let message_input = input("> "); 
        // if message_input == "up".to_string() {
        //     if (start > 2) { // prevent from going above the top
        //         start = start - 1;
        //     }
        // } else if message_input == "down".to_string() {
        //     if (start < messages_list.len() - 3) { // prevent from going below the bottom
        //         start = start + 1;
        //     }
        // } else if message_input == "back".to_string() {
        //     return true;
    //     } else {
    //         send_message_w_db(
    //             database.clone(),
    //             current_user_email.clone(),
    //             recipient_email.clone(),
    //             message_input.clone()
    //         ).await;
    //         break; // this loop breaks; the parent loop will create a new one

            // messages_list.push(Message {
            //     sender: current_user_email.to_string(),
            //     date_string: format!("{:?}", chrono::offset::Local::now()),
            //     content: message_input
            // });
            
            // if messages_list.len() > 3 {
            //     start = messages_list.len() - 3;
            // } else {
            //     start = 0;
            // }
    //     }
    //     // print!("{}[2J", 27 as char);

    // }
    // return Ok(());

}

fn print_messages(messages_list: &Vec<Message>, recipient_email: String, start: usize) {
    println!("\x1b[1mDirect Messages with {}\x1b[0m\n", recipient_email);
    println!("\n[back] Back to friends list");
    for i in max(0,start)..min(messages_list.len(), start+3) {
        println!("[{}, {}]", messages_list[i].sender, messages_list[i].date_string);
        println!("{}\n", messages_list[i].content);
    }
}

// macro_rules! do_loop {(
//     $body:block while $cond:expr
// ) => ({
//     let mut first = true;
//     while ::core::mem::replace(&mut first, false) || $cond
//         $body
// })}

// pub async fn messages(
//     database: Database,
//     current_user_email: String,
//     recipient_email: String
// ) -> mongodb::error::Result<()> {

//     let messages_coll: Collection<Document> = database.clone().collection("messages");
//     let mut change_stream = messages_coll.watch().await?;   
//     let mut full_break: bool = render_messages(database.clone(), current_user_email.clone(), recipient_email.clone()).await;
//     if (full_break) {
//         return Ok(());
//     }
    
//     while let Some(event) = change_stream.next().await.transpose()? { 
//         full_break = render_messages(database.clone(), current_user_email.clone(), recipient_email.clone()).await;
//         if (full_break) {
//             break;
//         }
//     }
//     return Ok(());
// }