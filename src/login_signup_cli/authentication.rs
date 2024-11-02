use mongodb::{ 
	bson::{Document, doc},
	Client,
	Collection 
};

//found out how to use these imports from https://hashing.ssojet.com/bcrypt-in-rust/
use bcrypt::verify;
use bcrypt::{hash, DEFAULT_COST};

#[derive(Debug, PartialEq)]
pub enum SignInOutcome {
    Success,
    UsernameNotFound,
    IncorrectPassword
}

/// Signs in users
/// Returns Ok(SignInOutcome) if username successfully queried in database. 
///     Success if username and correct password, 
///     IncorrectPassword if username and incorrect password, 
///     UsernameNotFound if username not found.
/// Returns Err(String) if mongodb or bcrypt error occurs
pub async fn sign_in(uri: String, username: String, password: String) -> Result<SignInOutcome, String> {
    // Create a new client and connect to the server
    let client = Client::with_uri_str(uri).await;
    if client.is_err() {
        return Err(client.unwrap_err().to_string());
    }
    // Get a database and sign in
    let database = client.unwrap().database("cli_chat");
    sign_in_w_db(database, username, password).await
}

/// Signs in users
/// Returns Ok(SignInOutcome) if username successfully queried in database. 
///     Success if username and correct password, 
///     IncorrectPassword if username and incorrect password, 
///     UsernameNotFound if username not found.
/// Returns Err(String) if mongodb or bcrypt error occurs
async fn sign_in_w_db(database: mongodb::Database, username: String, password: String) -> Result<SignInOutcome, String> {
    let user_coll: Collection<Document> = database.collection("users");

    // Find a user based on username
    let user = user_coll.find_one(doc! {"username": &username}).await;
    if user.is_err() {
        return Err(user.unwrap_err().to_string());
    }
    if ((&user).as_ref().unwrap()).is_none() {
        return Ok(SignInOutcome::UsernameNotFound);
    }

    let unwrapped_user = user.unwrap().unwrap();

    // Verify correct password
    let hashed_password: &str = unwrapped_user.get("password").unwrap().as_str().unwrap();
    let is_correct_password = verify(password, hashed_password);
    
    if is_correct_password.is_err() {
        Err(is_correct_password.unwrap_err().to_string())
    } else if is_correct_password.unwrap() {
        Ok(SignInOutcome::Success)
    } else {
        Ok(SignInOutcome::IncorrectPassword)
    }
}

/// Registers users
/// Returns Ok(bool) if username successfully queried in database. 
///     true if username does not initially exist and has been added, 
///     false if username does exist, 
/// Returns Err(String) if mongodb or bcrypt error occurs
pub async fn register_account(uri: String, username: String, password: String) -> Result<bool, String> {
    // Create a new client and connect to the server
    let client = Client::with_uri_str(uri).await;
    if client.is_err() {
        return Err(client.unwrap_err().to_string());
    }
    // Get database and register account
    let database = client.unwrap().database("cli_chat");
    return register_account_w_db(database, username, password).await
}

/// Registers users
/// Returns Ok(bool) if username successfully queried in database. 
///     true if username does not initially exist and has been added, 
///     false if username does exist, 
/// Returns Err(String) if mongodb or bcrypt error occurs
async fn register_account_w_db(database: mongodb::Database, username: String, password: String) -> Result<bool, String> {
    let user_coll: Collection<Document> = database.collection("users");

    // Find a user based on username
    let user = user_coll.find_one(doc! {"username": &username}).await;
    if user.is_err() {
        return Err(user.unwrap_err().to_string());
    }
    if ((&user).as_ref().unwrap()).is_some() {
        return Ok(false);
    }

    // Create and insert user document
    println!("Make new user.");
    let doc: Document = doc! { "username": username, "password": hash(password, DEFAULT_COST).unwrap()};
    let insert_one_result = user_coll.insert_one(doc).await;
    if insert_one_result.is_err() {
        return Err(insert_one_result.unwrap_err().to_string());
    }

    Ok(true)
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_register_account() {
        //remove account for test
        let client: Result<Client, mongodb::error::Error> = Client::with_uri_str("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0").await;

        let database = client.unwrap().database("cli_chat");
        let user_coll: Collection<Document> = database.collection("users");
        let filter = doc! { "username": "test0" }; 

        let result = user_coll.delete_one(filter).await;
        assert_eq!(result.unwrap().deleted_count, 1);

        //register account
        let register_account_result = register_account("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0".to_string(), "test0".to_string(), "test_pwd0".to_string()).await;
        assert_eq!(register_account_result.unwrap(), true);
    }
    #[tokio::test]
    async fn test_sign_in_correct_password() {
        let sign_in_result = sign_in("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0".to_string(), "test1".to_string(), "test_pwd1".to_string()).await;
        assert_eq!(sign_in_result.unwrap(), SignInOutcome::Success);
    }
    #[tokio::test]
    async fn test_sign_in_incorrect_password() {
        let sign_in_result = sign_in("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0".to_string(), "test1".to_string(), "incorrect".to_string()).await;
        assert_eq!(sign_in_result.unwrap(), SignInOutcome::IncorrectPassword);
    }
}