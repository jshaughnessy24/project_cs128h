
pub fn signup(email: String, password: String) -> Result<(), String> {
    if email == "test@gmail.com" {
        return Err("The email test@gmail.com is already taken.".to_string())
    }
    return Ok(());
}

pub fn login(email: String, password: String) -> Result<(), String> {
    if email == "test@gmail.com" {
        if password == "123456" {
            return Ok(());
        } else {
            return Err("The password was incorrect.".to_string())
        }
    } 
    return Err(format!("The email {} does not exist.", email));
}


