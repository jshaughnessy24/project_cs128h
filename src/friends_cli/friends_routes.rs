use mongodb::{
    bson::{doc, Bson, Document},
    Collection,
};

#[derive(Debug, PartialEq)]
pub enum AddFriendOutcome {
    Success,
    CurrentEmailNotFound,
    OtherEmailNotFound,
    AlreadyFriends,
}

#[derive(Debug, PartialEq)]
pub enum RemoveFriendOutcome {
    Success,
    CurrentEmailNotFound,
    OtherEmailNotFound,
    NotFriends,
}

/// Adds friends to inputted emails
/// Returns Ok(AddFriendOutcome).
///     Success if friends added to both users,
///     CurrentEmailNotFound if current user not found
///     OtherEmailNotFound if otehr user not found
///     AlreadyFriends if already friends
/// Returns Err(String) if mongodb error occurs
pub async fn add_friend_w_db(
    database: mongodb::Database,
    current_email: String,
    other_email: String,
) -> Result<AddFriendOutcome, String> {
    let user_coll: Collection<Document> = database.collection("users");

    // Find current user based on email
    let current_user_doc = match get_user_doc(&user_coll, &current_email).await {
        Ok(current_user_doc) => current_user_doc,
        Err(Ok(())) => return Ok(AddFriendOutcome::CurrentEmailNotFound),
        Err(Err(err_str)) => return Err(err_str),
    };

    // Get current user friends list
    let mut current_friends_vec = get_friend_vec_from_doc(&current_user_doc);

    // Check if already friends
    if current_friends_vec.contains(&other_email) {
        return Ok(AddFriendOutcome::AlreadyFriends);
    }

    // // Find other user based on email
    let other_user_doc = match get_user_doc(&user_coll, &other_email).await {
        Ok(other_user_doc) => other_user_doc,
        Err(Ok(())) => return Ok(AddFriendOutcome::OtherEmailNotFound),
        Err(Err(err_str)) => return Err(err_str),
    };

    // Get other user friends list
    let mut other_friends_vec = get_friend_vec_from_doc(&other_user_doc);

    // Create new friend lists
    current_friends_vec.push(other_email.clone());
    other_friends_vec.push(current_email.clone());

    // Update current friend list on mongodb
    let current_update_doc = doc! {
        "$set": doc! { "friends": current_friends_vec.as_slice()},
    };

    match user_coll
        .update_one(doc! { "email": current_email}, current_update_doc)
        .await
    {
        Ok(_) => (),
        Err(error) => return Err(error.to_string()),
    }

    // Update other friend list on mongodb
    let other_update_doc = doc! {
        "$set": doc! { "friends": other_friends_vec.as_slice()},
    };

    match user_coll
        .update_one(doc! { "email": other_email}, other_update_doc)
        .await
    {
        Ok(_) => (),
        Err(error) => return Err(error.to_string()),
    }

    Ok(AddFriendOutcome::Success)
}

/// Removes friends to inputted emails
/// Returns Ok(RemoveFriendOutcome).
///     Success if friends added to both users,
///     CurrentEmailNotFound if current user not found
///     OtherEmailNotFound if other user not found
///     NotFriends if not friends to begin with
/// Returns Err(String) if mongodb error occurs
pub async fn remove_friend_w_db(
    database: mongodb::Database,
    current_email: String,
    other_email: String,
) -> Result<RemoveFriendOutcome, String> {
    let user_coll: Collection<Document> = database.collection("users");

    // Find current user based on email
    let current_user_doc = match get_user_doc(&user_coll, &current_email).await {
        Ok(current_user_doc) => current_user_doc,
        Err(Ok(())) => return Ok(RemoveFriendOutcome::CurrentEmailNotFound),
        Err(Err(err_str)) => return Err(err_str),
    };

    // Get current user friends list
    let mut current_friends_vec = get_friend_vec_from_doc(&current_user_doc);

    // // Find other user based on email
    let other_user_doc = match get_user_doc(&user_coll, &other_email).await {
        Ok(other_user_doc) => other_user_doc,
        Err(Ok(())) => return Ok(RemoveFriendOutcome::OtherEmailNotFound),
        Err(Err(err_str)) => return Err(err_str),
    };

    // Check if not friends to begin with
    if !current_friends_vec.contains(&other_email) {
        return Ok(RemoveFriendOutcome::NotFriends);
    }

    // Get other user friends list
    let mut other_friends_vec = get_friend_vec_from_doc(&other_user_doc);

    // Create new friend lists
    let current_friends_index = current_friends_vec
        .iter()
        .position(|r| *r == other_email)
        .unwrap();
    current_friends_vec.remove(current_friends_index);

    let other_friends_index = other_friends_vec
        .iter()
        .position(|r| *r == current_email)
        .unwrap();
    other_friends_vec.remove(other_friends_index);

    // Update current friend list on mongodb
    let current_update_doc = doc! {
        "$set": doc! { "friends": current_friends_vec.as_slice()},
    };

    match user_coll
        .update_one(doc! { "email": current_email}, current_update_doc)
        .await
    {
        Ok(_) => (),
        Err(error) => return Err(error.to_string()),
    }

    // Update other friend list on mongodb
    let other_update_doc = doc! {
        "$set": doc! { "friends": other_friends_vec.as_slice()},
    };

    match user_coll
        .update_one(doc! { "email": other_email}, other_update_doc)
        .await
    {
        Ok(_) => (),
        Err(error) => return Err(error.to_string()),
    }

    Ok(RemoveFriendOutcome::Success)
}

/// Gets friend list for given email
/// Returns Ok(Some(Vec<String>)) if friends list obtained
/// Returns Ok(None) if user does not exist
/// Returns Error(String) if mongodb error occurs
pub async fn get_friend_list(
    database: mongodb::Database,
    email: String,
) -> Result<Option<Vec<String>>, String> {
    let user_coll: Collection<Document> = database.collection("users");

    // Find current user based on email
    let user_doc = match get_user_doc(&user_coll, &email).await {
        Ok(current_user_doc) => current_user_doc,
        Err(Ok(())) => return Ok(None),
        Err(Err(err_str)) => return Err(err_str),
    };

    Ok(Some(get_friend_vec_from_doc(&user_doc)))
}

/// Gets the document for a user given the user collection and the email
/// Returns Ok(Document) if user document obtained
/// Returns Err(Result<(), String>)
///     Ok() if email not found in database
///     Err(String) if mongodb error occurs
pub async fn get_user_doc(
    user_coll: &Collection<Document>,
    email: &String,
) -> Result<Document, Result<(), String>> {
    let user = user_coll.find_one(doc! {"email": &email}).await;
    if user.is_err() {
        return Err(Err(user.unwrap_err().to_string()));
    }
    if (&user).as_ref().unwrap().is_none() {
        return Err(Ok(()));
    }
    let user_doc = user.unwrap().unwrap();
    Ok(user_doc)
}

/// Gets friend vector given user doc
/// Returns Vec<String> the friend vector
pub fn get_friend_vec_from_doc(user_doc: &Document) -> Vec<String> {
    let current_friends_bson_vec: &Vec<Bson> = user_doc.get("friends").unwrap().as_array().unwrap();
    let current_friends_vec: Vec<String> = current_friends_bson_vec
        .iter()
        .map(|x| x.as_str().unwrap().to_string())
        .collect();
    current_friends_vec
}

#[cfg(test)]
mod test {
    use mongodb::Client;

    use super::*;

    #[tokio::test]
    async fn test_remove_friends_current_email_not_found() {
        //remove account for test
        let client: Result<Client, mongodb::error::Error> = Client::with_uri_str("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0").await;
        let database = client.unwrap().database("cli_chat");

        //remove friend
        let remove_friend_outcome =
            remove_friend_w_db(database, "".to_string(), "test2@test.com".to_string()).await;

        assert_eq!(
            remove_friend_outcome.unwrap(),
            RemoveFriendOutcome::CurrentEmailNotFound
        );
    }

    #[tokio::test]
    async fn test_remove_friends_other_email_not_found() {
        //remove account for test
        let client: Result<Client, mongodb::error::Error> = Client::with_uri_str("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0").await;
        let database = client.unwrap().database("cli_chat");

        //remove friend
        let remove_friend_outcome =
            remove_friend_w_db(database, "test2@test.com".to_string(), "".to_string()).await;

        assert_eq!(
            remove_friend_outcome.unwrap(),
            RemoveFriendOutcome::OtherEmailNotFound
        );
    }

    #[tokio::test]
    async fn test_remove_friends_not_friends() {
        //remove account for test
        let client: Result<Client, mongodb::error::Error> = Client::with_uri_str("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0").await;
        let database = client.unwrap().database("cli_chat");

        //remove friend
        let remove_friend_outcome = remove_friend_w_db(
            database,
            "test2@test.com".to_string(),
            "test5@test.com".to_string(),
        )
        .await;

        assert_eq!(
            remove_friend_outcome.unwrap(),
            RemoveFriendOutcome::NotFriends
        );
    }

    #[tokio::test]
    async fn test_remove_add_friends_success() {
        // get database
        let client: Result<Client, mongodb::error::Error> = Client::with_uri_str("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0").await;
        let database = client.unwrap().database("cli_chat");

        // remove friend
        let remove_friend_outcome = remove_friend_w_db(
            database.clone(),
            "test4@test.com".to_string(),
            "test5@test.com".to_string(),
        )
        .await;
        assert_eq!(remove_friend_outcome.unwrap(), RemoveFriendOutcome::Success);

        // add friend
        let add_friend_outcome = add_friend_w_db(
            database,
            "test4@test.com".to_string(),
            "test5@test.com".to_string(),
        )
        .await;
        assert_eq!(add_friend_outcome.unwrap(), AddFriendOutcome::Success);
    }

    #[tokio::test]
    async fn test_add_friends_current_email_not_found() {
        //remove account for test
        let client: Result<Client, mongodb::error::Error> = Client::with_uri_str("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0").await;
        let database = client.unwrap().database("cli_chat");

        //add friend
        let add_friend_outcome =
            add_friend_w_db(database, "".to_string(), "test2@test.com".to_string()).await;

        assert_eq!(
            add_friend_outcome.unwrap(),
            AddFriendOutcome::CurrentEmailNotFound
        );
    }

    #[tokio::test]
    async fn test_add_friends_other_email_not_found() {
        //remove account for test
        let client: Result<Client, mongodb::error::Error> = Client::with_uri_str("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0").await;
        let database = client.unwrap().database("cli_chat");

        //add friend
        let add_friend_outcome =
            add_friend_w_db(database, "test2@test.com".to_string(), "".to_string()).await;

        assert_eq!(
            add_friend_outcome.unwrap(),
            AddFriendOutcome::OtherEmailNotFound
        );
    }

    #[tokio::test]
    async fn test_add_friends_already_friends() {
        let client: Result<Client, mongodb::error::Error> = Client::with_uri_str("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0").await;
        let database = client.unwrap().database("cli_chat");

        //add friend
        let add_friend_outcome = add_friend_w_db(
            database,
            "test2@test.com".to_string(),
            "test3@test.com".to_string(),
        )
        .await;

        assert_eq!(
            add_friend_outcome.unwrap(),
            AddFriendOutcome::AlreadyFriends
        );
    }

    #[tokio::test]
    async fn test_get_friends_list_email_not_found() {
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
        let get_friend_list_outcome = get_friend_list(database, "test2@test.com".to_string()).await;

        //build friend vector
        let mut expected_friend_list: Vec<String> = Vec::new();
        expected_friend_list.push("test3@test.com".to_string());
        expected_friend_list.push("test4@test.com".to_string());
        assert_eq!(
            get_friend_list_outcome.unwrap().unwrap(),
            expected_friend_list
        );
    }
}
