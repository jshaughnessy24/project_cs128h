extern crate python_input;
use python_input::input;
// mod friends_routes;

pub fn friends() {
    // Define a mutable vector to hold friend usernames
    let mut friend_list: Vec<String> = Vec::new();

    loop {
        println!("\x1b[1mFriends List\x1b[0m\n");

        // Display friend list
        if friend_list.is_empty() {
            println!("You have no friends added.");
        } else {
            for (i, friend) in friend_list.iter().enumerate() {
                println!("{}: {}", i + 1, friend);
            }
        }

        println!("\nOptions:");
        println!("[back] Back to Homepage");
        println!("[add-friend] [name] Add a friend");
        println!("[remove-friend] [ID] Remove a friend");
        println!("[direct-message] [ID] DM friend\n");

        // Get the user input
        let choice = input("What would you like to do? ");

        // Parse the input for various commands
        let parts: Vec<&str> = choice.split_whitespace().collect();
        match parts.as_slice() {
            ["back"] => {
                println!("Returning to Homepage...");
                break;
            }
            ["add-friend", username] => {
                // friend_list.push(username.to_string());
                // println!("Friend '{}' added!", username);

                // match friends_routes::add_friend_w_db(email_input, password_input) {
                //     Ok(()) => {
                //         println!("Welcome to CLIChat!");
                //         break;
                //     }
                //     Err(e) => {
                //         println!("{}", e);
                //     }
                // }
            }

            // change to remove using username
            ["remove-friend", username] => {
                // if let Ok(index) = id.parse::<usize>() {
                //     if index > 0 && index <= friend_list.len() {
                //         let removed_friend = friend_list.remove(index - 1);
                //         println!("'{}' removed!", removed_friend);
                //     } else {
                //         println!("Invalid ID.");
                //     }
                // } else {
                //     println!("Please enter a valid ID.");
                // }
            }

            // change to dm using username
            ["direct-message", username] => {
                // if let Ok(index) = id.parse::<usize>() {
                //     if index > 0 && index <= friend_list.len() {
                //         println!("Direct messaging {}...", friend_list[index - 1]);
                //     } else {
                //         println!("Invalid ID.");
                //     }
                // } else {
                //     println!("Please enter a valid ID.");
                // }
            }
            _ => {
                println!("Invalid option, please try again.");
            }
        }

        println!();
    }
}
