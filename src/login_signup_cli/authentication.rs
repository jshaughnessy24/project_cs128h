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
    EmailNotFound,
    IncorrectPassword,
}

/// Signs in users
/// Returns Ok(SignInOutcome) if email successfully queried in database.
///     Success if email and correct password,
///     IncorrectPassword if email and incorrect password,
///     EmailNotFound if email not found.
/// Returns Err(String) if mongodb or bcrypt error occurs
pub async fn sign_in(
    uri: String,
    email: String,
    password: String,
) -> Result<SignInOutcome, String> {
    // Create a new client and connect to the server
    let client = Client::with_uri_str(uri).await;
    if client.is_err() {
        return Err(client.unwrap_err().to_string());
    }
    // Get a database and sign in
    let database = client.unwrap().database("cli_chat");
    sign_in_w_db(database, email, password).await
}

/// Signs in users
/// Returns Ok(SignInOutcome) if email successfully queried in database.
///     Success if email and correct password,
///     IncorrectPassword if email and incorrect password,
///     EmailNotFound if email not found.
/// Returns Err(String) if mongodb or bcrypt error occurs
async fn sign_in_w_db(
    database: mongodb::Database,
    email: String,
    password: String,
) -> Result<SignInOutcome, String> {
    let user_coll: Collection<Document> = database.collection("users");

    // Find a user based on email
    let user = user_coll.find_one(doc! {"email": &email}).await;
    if user.is_err() {
        return Err(user.unwrap_err().to_string());
    }
    if ((&user).as_ref().unwrap()).is_none() {
        return Ok(SignInOutcome::EmailNotFound);
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
/// Returns Ok(bool) if email successfully queried in database.
///     true if email does not initially exist and has been added,
///     false if email does exist,
/// Returns Err(String) if mongodb or bcrypt error occurs
pub async fn register_account(
    uri: String,
    email: String,
    password: String,
) -> Result<bool, String> {
    // Create a new client and connect to the server
    let client = Client::with_uri_str(uri).await;
    if client.is_err() {
        return Err(client.unwrap_err().to_string());
    }
    // Get database and register account
    let database = client.unwrap().database("cli_chat");
    register_account_w_db(database, email, password).await
}

/// Registers users
/// Returns Ok(bool) if email successfully queried in database.
///     true if email does not initially exist and has been added,
///     false if email does exist,
/// Returns Err(String) if mongodb or bcrypt error occurs
async fn register_account_w_db(
    database: mongodb::Database,
    email: String,
    password: String,
) -> Result<bool, String> {
    let user_coll: Collection<Document> = database.collection("users");

    // Find a user based on email
    let user = user_coll.find_one(doc! {"email": &email}).await;
    if user.is_err() {
        return Err(user.unwrap_err().to_string());
    }
    if ((&user).as_ref().unwrap()).is_some() {
        return Ok(false);
    }

    // Create and insert user document
    let doc: Document =
        doc! { "email": email, "password": hash(password, DEFAULT_COST).unwrap(), "friends": [], "group_chats": []};
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
        let filter = doc! { "email": "test0@test.com" };

        let result = user_coll.delete_one(filter).await;
        assert_eq!(result.unwrap().deleted_count, 1);

        //register account
        let register_account_result = register_account("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0".to_string(), "test0@test.com".to_string(), "test_pwd0".to_string()).await;
        assert_eq!(register_account_result.unwrap(), true);
    }
    #[tokio::test]
    async fn test_sign_in_correct_password() {
        let sign_in_result = sign_in("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0".to_string(), "test1@test.com".to_string(), "test_pwd1".to_string()).await;
        assert_eq!(sign_in_result.unwrap(), SignInOutcome::Success);
    }
    #[tokio::test]
    async fn test_sign_in_incorrect_password() {
        let sign_in_result = sign_in("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0".to_string(), "test1@test.com".to_string(), "incorrect".to_string()).await;
        assert_eq!(sign_in_result.unwrap(), SignInOutcome::IncorrectPassword);
    }
}
