
use std::cmp::max;
use std::cmp::min;
use std::thread;
extern crate python_input;
use python_input::input;
use super::messages_routes::{Message, get_messages, send_message_w_db};
use chrono;
use futures::TryStreamExt;
use futures::StreamExt;
use winit::event_loop::{EventLoop, EventLoopBuilder};
use winit::event::{Event, WindowEvent};

use mongodb::{
    bson::{self, doc, Bson, Document},
    Client, 
    Database, 
    Collection
};

enum CustomEvent {
    MongoDBChange(String)
}

async fn listen_for_changes(
    database: Database,
    tx: mpsc::Sender<CustomEvent>
) -> mongodb::error::Result<()> {
    let messages_coll: Collection<Document> = database.clone().collection("messages");
    let mut change_stream = messages_coll.watch().await?;   
    // TODO: Filter this stream to incoming messages only
    // TODO: Update the list of messages sent

    while let Some(event) = change_stream.next().await.transpose()? {
        println!("Operation performed: {:?}", event.operation_type);
        println!("Document: {:?}", event.full_document);
        tx.send(CustomEvent::MongoDBChange("TEST RECEIVED".to_string()));
    }
    return Ok(());

}

async fn run_change_stream(
    database: Database
) {
    // let collection = database.clone().collection("messages");

    // let mut change_stream = collection.watch().await?;   

    // while let Some(change) = change_stream.next().await {
    //     match change {
    //         Ok(event) => {
    //             println!("Change detected: {:?}", event);
    //             // Process the change event here
    //         }
    //         Err(e) => eprintln!("Error in change stream: {:?}", e),
    //     }
    // }
}

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

    let cloned_db = database.clone();

    let (tx, mut rx) = mpsc::channel(100);

    // listen_for_changes(cloned_db);
    tokio::spawn(listen_for_changes(cloned_db));

    let event_loop = EventLoop<CustomEvent> = EventLoopBuilder::with_user_event().build();
    let event_loop_proxy = event_loop.create_proxy();

    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            
        }
    })

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

    // open a new thread that watches for changes 


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

            messages_list.push(Message {
                sender: current_user_email.to_string(),
                date_string: format!("{:?}", chrono::offset::Local::now()),
                content: message_input
            });
            
            if messages_list.len() > 3 {
                start = messages_list.len() - 3;
            } else {
                start = 0;
            }
        }
        // print!("{}[2J", 27 as char);

    }
}