use chrono;

use mongodb::{
    bson::{doc, Bson, Document},
    Client, Collection,
};

#[derive(Debug, PartialEq)]
pub enum SendMessageOutcome {
    Success,
    AuthorEmailNotFound,
    OtherEmailNotFound,
    NotFriends
}

pub struct Message {
    sender: String,
    date_string: String,
    content: String
}

pub async fn send_message_w_db(
    database: mongodb::Database,
    author_email: String,
    recipient_email: String,
    content: String
) -> Result<SendMessageOutcome, String> {
    let messages_coll: Collection<Document> = database.collection("messages");
    let user_coll: Collection<Document> = database.collection("users");

    // Check that the author exists
    let current_user_doc = match friends_routes::get_user_doc(&user_coll, &current_email).await {
        Ok(current_user_doc) => current_user_doc,
        Err(Ok(())) => return Ok(AddFriendOutcome::CurrentEmailNotFound),
        Err(Err(err_str)) => return Err(err_str)
    };

    // Check that author is friends with the recipient
    let mut current_friends_vec = friends_routes::get_friend_vec_from_doc(&current_user_doc);

    if !current_friends_vec.contains(&other_email) {
        return Ok(AddFriendOutcome::NotFriends);
    }

    // Create a new message, with: message content, date sent, author email, recipient email
    let doc: Document = 
        doc! { 
            "author_email": author_email, 
            "message_content": content, 
            "date_sent": chrono::offset::Local::now(),
            "recipient_email": recipient_email 
        }
    let insert_one_result = messages_coll.insert_one(doc).await;
    if insert_one_result.is_err() {
        return Err(insert_one_result.unwrap_err().to_string());
    }

    Ok(SendMessageOutcome::Success);
}

// pub async fn get_messages(
//     database: mongodb::Database,
//     author_email: String,
//     recipient_email: String
// ) -> Result<Option<Vec<Message>>, String> {
//     let messages_coll: Collection<Document> = database.collection("messages");

//     let messages = messages_coll.find(
//         doc! {
//             "author_email": author_email.to_string(),
//             "recipient_email": recipient_email.to_string()
//         }
//     );


// }

mod test {
    async fn test_message_to_non_friend() {
        // This should not work
    }
    async fn test_message_to_friend() {
       // This should work
       let client: Result<Client, mongodb::error::Error> = Client::with_uri_str("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0").await;
       let database = client.unwrap().database("cli_chat");

       // TODO: Create these users
       let send_message_outcome = send_message_w_db(
        database,
        "test@test.com",
        "test2@test.com"
       )

       assert_eq!(send_message_outcome.unwrap(), SendMessageOutcome::Success)

    }
    async fn test_retrieve_messages() {
        // ensure that it only gets the messages for the group chat btw. the two users
    }
}