
use std::cmp::max;
use std::cmp::min;
extern crate python_input;
use python_input::input;

struct Message {
    sender: String,
    date_string: String,
    content: String
}

pub async fn message_cli(
    recipient_email: String
) {
    // TODO: Retrieve messages

    let mut messages: Vec<Message> = Vec::new();
    messages.push(Message {
        sender: "test@test.com".to_string(),
        date_string: "11/1/2024 11:50pm".to_string(),
        content: "Hello!".to_string()
    });
    messages.push(Message {
        sender: "tes2t@test.com".to_string(),
        date_string: "11/2/2024 11:50pm".to_string(),
        content: "Hello!".to_string()
    });
    messages.push(Message {
        sender: "test3@test.com".to_string(),
        date_string: "11/3/2024 11:50pm".to_string(),
        content: "Hello!".to_string()
    });
    messages.push(Message {
        sender: "test@test.com".to_string(),
        date_string: "11/4/2024 11:50pm".to_string(),
        content: "Hello!".to_string()
    });
    messages.push(Message {
        sender: "tes2t@test.com".to_string(),
        date_string: "11/5/2024 11:50pm".to_string(),
        content: "Hello!".to_string()
    });
    messages.push(Message {
        sender: "test3@test.com".to_string(),
        date_string: "11/6/2024 11:50pm".to_string(),
        content: "Hello!".to_string()
    });
    messages.push(Message {
        sender: "test@test.com".to_string(),
        date_string: "11/7/2024 11:50pm".to_string(),
        content: "Hello!".to_string()
    });
    messages.push(Message {
        sender: "tes2t@test.com".to_string(),
        date_string: "11/8/2024 11:50pm".to_string(),
        content: "Hello!".to_string()
    });
    messages.push(Message {
        sender: "test3@test.com".to_string(),
        date_string: "11/9/2024 11:50pm".to_string(),
        content: "Hello!".to_string()
    });
    
    let mut start = messages.len() - 3;

    loop {
        println!("\x1b[1mDirect Messages with {}\x1b[0m\n", recipient_email);
        println!("\n[back] Back to friends list");
        for i in max(0,start)..min(messages.len(), start+3) {
            println!("[{}, {}]", messages[i].sender, messages[i].date_string);
            println!("{}\n", messages[i].content);
        }
        // TODO: 
        let option = input("What would you like to do? ");
        if option == "up".to_string() {
            start = start - 1;
        } else {
            start = start + 1;
        }
        print!("{}[2J", 27 as char);

    }
}