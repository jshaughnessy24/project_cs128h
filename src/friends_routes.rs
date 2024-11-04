use mongodb::{
    bson::{doc, Bson, Document},
    Client, Collection,
};

#[derive(Debug, PartialEq)]
pub enum AddFriendOutcome {
    Success,
    CurrentUsernameNotFound,
    OtherUsernameNotFound,
    AlreadyFriends
}

/// Adds friends to inputted usernames
/// Returns Ok(AddFriendOutcome).
///     Success if friends added to both users,
///     CurrentUsernameNotFound if current user not found
///     OtherUsernameNotFound if otehr user not found
///     AlreadyFriends if already friends
/// Returns Err(String) if mongodb error occurs
pub async fn add_friend_w_db(
    database: mongodb::Database,
    current_username: String,
    other_username: String,
) -> Result<AddFriendOutcome, String> {
    let user_coll: Collection<Document> = database.collection("users");

    // Find current user based on username
    let current_user_doc = match get_user_doc(&user_coll, &current_username).await {
        Ok(current_user_doc) => current_user_doc,
        Err(Ok(())) => return Ok(AddFriendOutcome::CurrentUsernameNotFound),
        Err(Err(err_str)) => return Err(err_str)
    };

    // Get current user friends list
    let mut current_friends_vec = get_friend_vec_from_doc(&current_user_doc);

    // Check if already friends
    if current_friends_vec.contains(&other_username) {
        return Ok(AddFriendOutcome::AlreadyFriends);
    } 

    // // Find other user based on username
    let other_user_doc = match get_user_doc(&user_coll, &other_username).await {
        Ok(other_user_doc) => other_user_doc,
        Err(Ok(())) => return Ok(AddFriendOutcome::OtherUsernameNotFound),
        Err(Err(err_str)) => return Err(err_str)
    };

    // Get other user friends list
    let mut other_friends_vec = get_friend_vec_from_doc(&other_user_doc);

    // Create new friend lists
    current_friends_vec.push(other_username.clone());
    other_friends_vec.push(current_username.clone());

    // Update current friend list on mongodb
    let current_update_doc = doc! {
        "$set": doc! { "friends": current_friends_vec.as_slice()},
    };

    match user_coll
    .update_one(doc! { "username": current_username}, current_update_doc)
    .await {
        Ok(_) => (),
        Err(error) => return Err(error.to_string())
    }
    
    // Update other friend list on mongodb
    let other_update_doc = doc! {
        "$set": doc! { "friends": other_friends_vec.as_slice()},
    };

    match user_coll
    .update_one(doc! { "username": other_username}, other_update_doc)
    .await {
        Ok(_) => (),
        Err(error) => return Err(error.to_string())
    }

    Ok(AddFriendOutcome::Success)
}

/// Gets friend list for given username
/// Returns Ok(Some(Vec<String>)) if friends list obtained
/// Returns Ok(None) if user does not exist
/// Returns Error(String) if mongodb error occurs
pub async fn get_friend_list(database: mongodb::Database, username: String) -> Result<Option<Vec<String>>, String> {
    let user_coll: Collection<Document> = database.collection("users");

    // Find current user based on username
    let user_doc = match get_user_doc(&user_coll, &username).await {
        Ok(current_user_doc) => current_user_doc,
        Err(Ok(())) => return Ok(None),
        Err(Err(err_str)) => return Err(err_str)
    };

    Ok(Some(get_friend_vec_from_doc(&user_doc)))
}

/// Gets the document for a user given the user collection and the username
/// Returns Ok(Document) if user document obtained
/// Returns Err(Result<(), String>)
///     Ok() if username not found in database
///     Err(String) if mongodb error occurs
async fn get_user_doc(user_coll: &Collection<Document>, username: &String) -> Result<Document, Result<(), String>> {
    let user = user_coll.find_one(doc! {"username": &username}).await;
    if user.is_err() {
        return Err(Err(user.unwrap_err().to_string()));
    }
    if ((&user).as_ref().unwrap()).is_none() {
        return Err(Ok(()));
    }
    let user_doc = user.unwrap().unwrap();
    Ok(user_doc)
}

/// Gets friend vector given user doc
/// Returns Vec<String> the friend vector
fn get_friend_vec_from_doc(user_doc: &Document) -> Vec<String> {
    let current_friends_bson_vec: &Vec<Bson> = user_doc.get("friends").unwrap().as_array().unwrap();
    let current_friends_vec: Vec<String> = current_friends_bson_vec.iter().map(|x| x.as_str().unwrap().to_string()).collect();
    current_friends_vec
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_add_friends_current_username_not_found() {
        //remove account for test
        let client: Result<Client, mongodb::error::Error> = Client::with_uri_str("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0").await;
        let database = client.unwrap().database("cli_chat");

        //add friend
        let add_friend_outcome = add_friend_w_db(database, "".to_string(), "test2".to_string()).await;

        assert_eq!(add_friend_outcome.unwrap(), AddFriendOutcome::CurrentUsernameNotFound);
    }

    #[tokio::test]
    async fn test_add_friends_other_username_not_found() {
        //remove account for test
        let client: Result<Client, mongodb::error::Error> = Client::with_uri_str("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0").await;
        let database = client.unwrap().database("cli_chat");

        //add friend
        let add_friend_outcome = add_friend_w_db(database, "test2".to_string(), "".to_string()).await;

        assert_eq!(add_friend_outcome.unwrap(), AddFriendOutcome::OtherUsernameNotFound);
    }

    #[tokio::test]
    async fn test_add_friends_already_friends() {
        let client: Result<Client, mongodb::error::Error> = Client::with_uri_str("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0").await;
        let database = client.unwrap().database("cli_chat");

        //add friend
        let add_friend_outcome = add_friend_w_db(database, "test2".to_string(), "test3".to_string()).await;

        assert_eq!(add_friend_outcome.unwrap(), AddFriendOutcome::AlreadyFriends);
    }

    #[tokio::test]
    async fn test_add_friends_success() {
        // reset friends lists
        let client: Result<Client, mongodb::error::Error> = Client::with_uri_str("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0").await;

        let database = client.unwrap().database("cli_chat");
        let user_coll: Collection<Document> = database.collection("users");
        // Reset test4 friend list on mongodb
        let test4_update_doc = doc! {
            "$set": doc! { "friends": ["test2".to_string()]},
        };

        let update_test4_res = user_coll.update_one(doc! { "username": "test4".to_string()}, test4_update_doc).await;
        assert!(update_test4_res.is_ok());

        // Reset test5 friend list on mongodb
        let test5_update_doc = doc! {
            "$set": doc! { "friends": []},
        };

        let update_test5_res = user_coll.update_one(doc! { "username": "test5".to_string()}, test5_update_doc).await;
        assert!(update_test5_res.is_ok());

        // add friend
        let add_friend_outcome = add_friend_w_db(database, "test4".to_string(), "test5".to_string()).await;
        assert_eq!(add_friend_outcome.unwrap(), AddFriendOutcome::Success);
    }

    #[tokio::test]
    async fn test_get_friends_list_username_not_found() {
        // get database
        let client: Result<Client, mongodb::error::Error> = Client::with_uri_str("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0").await;
        let database = client.unwrap().database("cli_chat");

        //add friend
        let add_friend_outcome = get_friend_list(database, "".to_string()).await;

        assert_eq!(add_friend_outcome.unwrap(), None);
    }

    #[tokio::test]
    async fn test_get_friends_list() {
        // get database
        let client: Result<Client, mongodb::error::Error> = Client::with_uri_str("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0").await;
        let database = client.unwrap().database("cli_chat");

        //add friend
        let get_friend_list_outcome = get_friend_list(database, "test2".to_string()).await;

        //build friend vector
        let mut expected_friend_list: Vec<String> = Vec::new();
        expected_friend_list.push("test3".to_string());
        expected_friend_list.push("test4".to_string());
        assert_eq!(get_friend_list_outcome.unwrap().unwrap(), expected_friend_list);
    }
}