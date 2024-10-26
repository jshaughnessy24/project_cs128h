extern crate python_input;
use python_input::input;

pub fn loginSignupCli() {
    // println!("Welcome to CLIChat!\n");
    // println!("[1] Login");
    // println!("[2] Sign Up");

    // loop {
    //     // Get input from the user
    //     let input : String = Input::new()
    //         .with_prompt(">")
    //         .interact_text()?;

    //     // Split the command line input by spaces
    //     let args : Vec<&str> = input.trim().split(' ').collect();

    //     println!("{:?}", args);
    // }


  let name = input("What is your name? ");
  let age = input("How old are you? ");

  println!("Hello {}, you are {} years old.", name, age);
}
