use colored::Colorize;

extern crate python_input;
use python_input::input;

pub fn new_group_chat(current_user_email: String) -> Option<(String, String)> {
    println!("\x1b[1mCreate New Group Chat\x1b[0m");
    println!();
    println!("[back] Back to Group Chats");

    println!();
    let chat_name = input("Chat Name: ");
    let friends_to_add = input("Friends to add: ");
    println!();

    println!("{}", "Group chat created.".green().bold());

    Some((chat_name, friends_to_add))
}
