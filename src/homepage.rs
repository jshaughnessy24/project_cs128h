extern crate python_input;
use python_input::input;

pub fn homepage() {
    println!("{}[2J", 27 as char);
    println!("\x1b[1mHomepage\x1b[0m");
    println!();
    println!("[1] Friends List");
    println!("[2] Group Chats");
    println!();

    let choice = input("Which would you like to access? ");

    // println!("You chose: {}", choice);
}
