use std::cmp::max;
use std::cmp::min;
extern crate python_input;
use python_input::input;

struct Message {
    sender: String,
    date_string: String,
    content: String,
}

pub async fn group_message(group_name: String) {
    // TODO: Retrieve messages

    let mut messages: Vec<Message> = Vec::new();
    messages.push(Message {
        sender: "test@test.com".to_string(),
        date_string: "11/1/2024 11:50pm".to_string(),
        content: "Hello!".to_string(),
    });
    messages.push(Message {
        sender: "tes2t@test.com".to_string(),
        date_string: "11/2/2024 11:50pm".to_string(),
        content: "Hello!".to_string(),
    });
    messages.push(Message {
        sender: "test3@test.com".to_string(),
        date_string: "11/3/2024 11:50pm".to_string(),
        content: "Hello!".to_string(),
    });
    messages.push(Message {
        sender: "test@test.com".to_string(),
        date_string: "11/4/2024 11:50pm".to_string(),
        content: "Hello!".to_string(),
    });
    messages.push(Message {
        sender: "tes2t@test.com".to_string(),
        date_string: "11/5/2024 11:50pm".to_string(),
        content: "Hello!".to_string(),
    });
    messages.push(Message {
        sender: "test3@test.com".to_string(),
        date_string: "11/6/2024 11:50pm".to_string(),
        content: "Hello!".to_string(),
    });
    messages.push(Message {
        sender: "test@test.com".to_string(),
        date_string: "11/7/2024 11:50pm".to_string(),
        content: "Hello!".to_string(),
    });
    messages.push(Message {
        sender: "tes2t@test.com".to_string(),
        date_string: "11/8/2024 11:50pm".to_string(),
        content: "Hello!".to_string(),
    });
    messages.push(Message {
        sender: "test3@test.com".to_string(),
        date_string: "11/9/2024 11:50pm".to_string(),
        content: "Hello!".to_string(),
    });

    let mut start = messages.len() - 3;

    loop {
        println!("\x1b[1mGroup Chat: {}\x1b[0m\n", group_name);
        println!("\n[back] Back to friends list");
        for i in max(0, start)..min(messages.len(), start + 3) {
            println!("[{}, {}]", messages[i].sender, messages[i].date_string);
            println!("{}\n", messages[i].content);
        }
        println!("Submit your message, or navigate by typing up or down.");
        let message_input = input("> ");
        if message_input == "up".to_string() {
            if (start > 2) {
                // prevent from going above the top
                start = start - 1;
            }
        } else if message_input == "down".to_string() {
            if (start < messages.len() - 3) {
                // prevent from going below the bottom
                start = start + 1;
            }
        } else {
            messages.push(Message {
                sender: current_user_email.to_string(),
                date_string: "11/9/2024 11:50pm".to_string(),
                content: message_input,
            });
            start = start + 1;
        }
        print!("{}[2J", 27 as char);
    }
}
