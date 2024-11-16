extern crate python_input;
use python_input::input;

pub fn group_chat(current_user_email: String, group_name: String) -> String {
    println!("\x1b[1mGroup Chats\x1b[0m");
    println!();
    println!("[back] Back to Hompepage");
    println!("[new] New Group Chat");

    println!();
    println!("Group Chat List: ");
    println!("[1] Friends List");
    println!("[2] Group Chats");
    println!();

    let choice = input("Which chat would you like to access? ");

    choice
}
