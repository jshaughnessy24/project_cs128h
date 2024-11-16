extern crate python_input;
use python_input::input;

pub fn new_group_chat() -> Option<String, String> {
    println!("\x1b[1mCreate New Group Chat\x1b[0m");
    println!();
    println!("[back] Back to Group Chats");

    println!();
    let chat_name = input("Chat Name: ");
    let friends_to_add = input("Friends to add: ");
    println!("[1] Friends List");
    println!("[2] Group Chats");
    println!();

    Ok(chat_name, friends_to_add)
}
