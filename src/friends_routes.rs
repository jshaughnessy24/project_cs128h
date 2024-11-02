use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};

//found out how to use these imports from https://hashing.ssojet.com/bcrypt-in-rust/
use bcrypt::verify;
use bcrypt::{hash, DEFAULT_COST};

#[derive(Debug, PartialEq)]
pub enum SignInOutcome {
    Success,
    UsernameNotFound,
    IncorrectPassword,
}

/// Registers users
/// Returns Ok(bool) if username successfully queried in database.
///     true if username does not initially exist and has been added,
///     false if username does exist,
/// Returns Err(String) if mongodb or bcrypt error occurs
pub async fn add_friend(uri: String, username: String, password: String) -> Result<bool, String> {
    // Create a new client and connect to the server
    let client = Client::with_uri_str(uri).await;
    if client.is_err() {
        return Err(client.unwrap_err().to_string());
    }
    // Get database and register account
    let database = client.unwrap().database("cli_chat");
    register_account_w_db(database, username, password).await
}

/// Registers users
/// Returns Ok(bool) if username successfully queried in database.
///     true if username does not initially exist and has been added,
///     false if username does exist,
/// Returns Err(String) if mongodb or bcrypt error occurs
async fn add_friend_w_db(
    database: mongodb::Database,
    username: String,
    password: String,
) -> Result<bool, String> {
    let user_coll: Collection<Document> = database.collection("users");

    // Find a user based on username
    let user = user_coll.find_one(doc! {"username": &username}).await;
    if user.is_err() {
        return Err(user.unwrap_err().to_string());
    }
    if ((&user).as_ref().unwrap()).is_none()) {
        return Ok(false);
    }

    // Create and insert user document
    let doc: Document = doc! { "username": username, "password": hash(password, DEFAULT_COST).unwrap(), "friends": [], "group_chats": []};
    let insert_one_result = user_coll.insert_one(doc).await;
    if insert_one_result.is_err() {
        return Err(insert_one_result.unwrap_err().to_string());
    }

    Ok(true)
}
