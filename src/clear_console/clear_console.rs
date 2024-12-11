/// Clears the console output.
pub fn clear_console() {
    println!("{}[2J", 27 as char);
}