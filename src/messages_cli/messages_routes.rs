use futures::TryStreamExt;
use mongodb::{
    bson::{self, doc, Bson, Document},
    Client, Collection,
};

use crate::friends_cli::friends_routes;

#[derive(Debug, PartialEq)]
pub enum SendMessageOutcome {
    Success,
    AuthorEmailNotFound,
    OtherEmailNotFound,
    NotFriends
}

#[derive(Debug, PartialEq)]
pub struct Message {
    pub sender: String,
    pub date_string: String,
    pub content: String
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
    let author_user_doc = match friends_routes::get_user_doc(&user_coll, &author_email).await {
        Ok(author_user_doc) => author_user_doc,
        Err(Ok(())) => return Ok(SendMessageOutcome::AuthorEmailNotFound),
        Err(Err(err_str)) => return Err(err_str)
    };

    // Check that author is friends with the recipient
    let mut author_friends_vec = friends_routes::get_friend_vec_from_doc(&author_user_doc);

    if !author_friends_vec.contains(&recipient_email) {
        return Ok(SendMessageOutcome::NotFriends);
    }

    // Create a new message, with: message content, date sent, author email, recipient email
    let doc: Document = 
        doc! { 
            "author_email": author_email, 
            "message_content": content, 
            "date_sent": bson::DateTime::now(),
            "recipient_email": recipient_email 
        };

    let insert_one_result = messages_coll.insert_one(doc).await;
    if insert_one_result.is_err() {
        return Err(insert_one_result.unwrap_err().to_string());
    }

    Ok(SendMessageOutcome::Success)
}

/// returns message vector of messages from author email to recipient email in no particular order. Must be sorted.
pub async fn get_messages(
    database: mongodb::Database,
    author_email: String,
    recipient_email: String
) -> Result<Option<Vec<Message>>, String> {
    let messages_coll: Collection<Document> = database.collection("messages");

    let mut messages = messages_coll.find(
        doc! {
            "author_email": author_email.to_string(),
            "recipient_email": recipient_email.to_string()
        }
    ).await;

    let mut message_vec: Vec<Message> = Vec::new();

    while let Some(doc) = messages.as_mut().unwrap().try_next().await.unwrap() {
        message_vec.push(Message{sender:doc.get("author_email").unwrap().as_str().unwrap().to_string(), 
        date_string: doc.get("date_sent").unwrap().as_datetime().unwrap().to_string(), 
        content: doc.get("message_content").unwrap().as_str().unwrap().to_string()});
    }

    Ok(Some(message_vec))
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_message_to_non_friend() {
        // This should not work
        let client: Result<Client, mongodb::error::Error> = Client::with_uri_str("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0").await;
        let database = client.unwrap().database("cli_chat");

       // TODO: Create these users
       let send_message_outcome = send_message_w_db(
        database,
        "test@test.com".to_string(),
        "".to_string(), 
        "hello".to_string()
       ).await;

       assert_eq!(send_message_outcome.unwrap(), SendMessageOutcome::NotFriends)
    }

    #[tokio::test]
    async fn test_message_to_friend() {
       // This should work
       let client: Result<Client, mongodb::error::Error> = Client::with_uri_str("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0").await;
       let database = client.unwrap().database("cli_chat");

        let message_coll: Collection<Document> = database.collection("messages");
        let filter = doc! { "author_email": "test6@test.com" };

        let result = message_coll.delete_one(filter).await;
        assert_eq!(result.unwrap().deleted_count, 1);

       let send_message_outcome = send_message_w_db(
        database,
        "test6@test.com".to_string(),
        "test7@test.com".to_string(), 
        "hello".to_string()
       ).await;
       assert_eq!(send_message_outcome.unwrap(), SendMessageOutcome::Success)
    }

    #[tokio::test]
    async fn test_message_author_email_not_found() {
        // This should not work
        let client: Result<Client, mongodb::error::Error> = Client::with_uri_str("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0").await;
        let database = client.unwrap().database("cli_chat");

       let send_message_outcome = send_message_w_db(
        database,
        "".to_string(),
        "test@test.com".to_string(), 
        "hello".to_string()
       ).await;

       assert_eq!(send_message_outcome.unwrap(), SendMessageOutcome::AuthorEmailNotFound)
    }

    #[tokio::test]
    async fn test_retrieve_messages() {
        // ensure that it only gets the messages for the group chat btw. the two users
        let client: Result<Client, mongodb::error::Error> = Client::with_uri_str("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0").await;
        let database = client.unwrap().database("cli_chat");

        let messages_vec = get_messages(database, "test4@test.com".to_string(), "test5@test.com".to_string()).await.unwrap().unwrap();

        assert_eq!(messages_vec.len(), 1);
        assert!(messages_vec.contains(&Message{sender: "test4@test.com".to_string(), date_string: "2024-11-15 7:00:22.399 +00:00:00".to_string(), content: "testing message".to_string()}))
    }
}