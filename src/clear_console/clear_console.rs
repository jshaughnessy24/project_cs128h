/// Clears the console output.
pub fn clear_console() {
    print!("{}[2J", 27 as char);
}