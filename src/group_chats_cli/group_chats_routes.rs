use std::collections::HashMap;

use futures::TryStreamExt;
use mongodb::{
    bson::{self, doc, oid::ObjectId, Bson, Document}, Collection,
};

use crate::friends_cli::friends_routes::{self};

#[derive(Debug, PartialEq)]
pub enum AddGroupChatOutcome {
    Success(ObjectId),
    CreatorEmailNotFound,
    SomeEmailNotFound(String),
    SomeEmailNotFriends(String),
}

#[derive(Debug, PartialEq)]
pub enum SendMessageOutcome {
    Success,
    AuthorEmailNotFound,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Message {
    pub sender: String,
    pub date_string: String,
    pub content: String
}

pub async fn send_message_group_chat_w_db(
    database: mongodb::Database,
    author_email: String,
    group_chat_id: ObjectId,
    content: String,
) -> Result<SendMessageOutcome, String> {
    let messages_coll: Collection<Document> = database.collection("messages");
    let user_coll: Collection<Document> = database.collection("users");

    // Check that the author exists
    match friends_routes::get_user_doc(&user_coll, &author_email).await {
        Ok(_) => {}
        Err(Ok(())) => return Ok(SendMessageOutcome::AuthorEmailNotFound),
        Err(Err(err_str)) => return Err(err_str),
    };

    // Create a new message, with: message content, date sent, author email, recipient email
    let doc: Document = doc! {
        "author_email": author_email,
        "message_content": content,
        "date_sent": bson::DateTime::now(),
        "group_chat_id": group_chat_id
    };

    let insert_one_result = messages_coll.insert_one(doc).await;
    if insert_one_result.is_err() {
        return Err(insert_one_result.unwrap_err().to_string());
    }

    Ok(SendMessageOutcome::Success)
}

/// returns message vector of messages in group chat in no particular order. Must be sorted.
pub async fn get_messages_group_chat(
    database: mongodb::Database,
    group_chat_id: ObjectId,
) -> Result<Option<Vec<Message>>, String> {
    let messages_coll: Collection<Document> = database.collection("messages");

    let mut messages = messages_coll
        .find(doc! {
            "group_chat_id": group_chat_id
        })
        .await;

    let mut message_vec: Vec<Message> = Vec::new();

    while let Some(doc) = messages.as_mut().unwrap().try_next().await.unwrap() {
        message_vec.push(Message {
            sender: doc
                .get("author_email")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            date_string: doc
                .get("date_sent")
                .unwrap()
                .as_datetime()
                .unwrap()
                .to_string(),
            content: doc
                .get("message_content")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
        });
    }

    Ok(Some(message_vec))
}

/// Adds group chat
/// Returns Ok(AddGroupChatOutcome) if email successfully queried in database.
///     CreatorEmailNotFound if creator email not found,
///     SomeEmailNotFound(String) if some email does not exist,
///     SomeEmailNotFriends(String) if some email not friends
/// Returns Err(String) if mongodb error occurs
pub async fn add_group_chat_w_db(
    database: mongodb::Database,
    group_chat_name: String,
    creator_email: String,
    friends_emails: Vec<String>,
) -> Result<AddGroupChatOutcome, String> {
    let user_coll: Collection<Document> = database.collection("users");

    // Check that the author exists
    let creator_user_doc = match friends_routes::get_user_doc(&user_coll, &creator_email).await {
        Ok(creator_user_doc) => creator_user_doc,
        Err(Ok(())) => return Ok(AddGroupChatOutcome::CreatorEmailNotFound),
        Err(Err(err_str)) => return Err(err_str),
    };

    // Check author friends with all in friends_emails, collects friends_user_docs
    let creator_friends_vec = friends_routes::get_friend_vec_from_doc(&creator_user_doc);
    let mut friends_user_docs: Vec<Document> = Vec::new();

    for friend_email in friends_emails.as_slice() {
        friends_user_docs.push(
            match friends_routes::get_user_doc(&user_coll, friend_email).await {
                Ok(friends_user_doc) => friends_user_doc,
                Err(Ok(())) => {
                    return Ok(AddGroupChatOutcome::SomeEmailNotFound(
                        friend_email.to_string(),
                    ))
                }
                Err(Err(err_str)) => return Err(err_str),
            },
        );
        if !creator_friends_vec.contains(friend_email) {
            return Ok(AddGroupChatOutcome::SomeEmailNotFriends(
                friend_email.to_string(),
            ));
        }
    }

    // Create and insert groupchat document
    let group_chat_coll: Collection<Document> = database.collection("group_chats");

    let mut group_chat_emails = friends_emails;
    group_chat_emails.push(creator_email.clone());

    let group_chat_doc: Document =
        doc! { "name": group_chat_name, "members": group_chat_emails.as_slice()};
    let insert_one_result = group_chat_coll.insert_one(group_chat_doc).await;
    if insert_one_result.is_err() {
        return Err(insert_one_result.unwrap_err().to_string());
    }
    let group_chat_id = insert_one_result
        .unwrap()
        .inserted_id
        .as_object_id()
        .unwrap();

    // Add groupchat id to creator
    let mut creator_group_chat_vec = get_group_chat_vec_from_doc(&creator_user_doc);
    creator_group_chat_vec.push(group_chat_id.clone());

    // Update creator group chat list on mongodb
    let creator_update_doc = doc! {
        "$set": doc! { "group_chats": creator_group_chat_vec.as_slice()},
    };

    match user_coll
        .update_one(creator_user_doc, creator_update_doc)
        .await
    {
        Ok(_) => (),
        Err(error) => return Err(error.to_string()),
    }

    // add groupchat id to friends
    for friend_doc in friends_user_docs {
        // Add groupchat id to friend
        let mut friend_group_chat_vec = get_group_chat_vec_from_doc(&friend_doc);
        friend_group_chat_vec.push(group_chat_id.clone());

        // Update creator group chat list on mongodb
        let friend_update_doc = doc! {
            "$set": doc! { "group_chats": friend_group_chat_vec.as_slice()},
        };

        match user_coll.update_one(friend_doc, friend_update_doc).await {
            Ok(_) => (),
            Err(error) => return Err(error.to_string()),
        }
    }

    Ok(AddGroupChatOutcome::Success(group_chat_id))
}

/// Gets group chat vector given user doc
/// Returns Vec<String> the friend vector
pub fn get_group_chat_vec_from_doc(user_doc: &Document) -> Vec<ObjectId> {
    let current_group_chat_bson_vec: &Vec<Bson> =
        user_doc.get("group_chats").unwrap().as_array().unwrap();
    let current_group_chat_vec: Vec<ObjectId> = current_group_chat_bson_vec
        .iter()
        .map(|x| x.as_object_id().unwrap())
        .collect();
    current_group_chat_vec
}

/// For a given email, gets a hashmap of group chat ids mapped to the group chat names
pub async fn get_group_chat_ids_names_map(
    database: mongodb::Database,
    email: String,
) -> Result<Option<HashMap<ObjectId, String>>, String> {
    let mut id_to_name_map: HashMap<ObjectId, String> = HashMap::new();
    let user_coll: Collection<Document> = database.collection("users");
    let group_chat_coll: Collection<Document> = database.collection("group_chats");

    // Check that the author exists
    let user_doc = match friends_routes::get_user_doc(&user_coll, &email).await {
        Ok(user_doc) => user_doc,
        Err(Ok(())) => return Ok(None),
        Err(Err(err_str)) => return Err(err_str),
    };

    let group_chat_vec = get_group_chat_vec_from_doc(&user_doc);
    for group_chat_id in group_chat_vec {
        let group_chat_doc = match group_chat_coll.find_one(doc! {"_id": group_chat_id}).await {
            Ok(Some(group_chat_doc)) => group_chat_doc,
            Ok(None) => return Err("no group chat document".to_string()),
            Err(error) => return Err(error.to_string()),
        };

        id_to_name_map.insert(
            group_chat_id,
            group_chat_doc
                .get("name")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
        );
    }

    Ok(Some(id_to_name_map))
}

mod test {
    

    
    #[tokio::test]

    async fn test_add_group_chat_current_email_not_found() {
        //get database for test
        let client: Result<Client, mongodb::error::Error> = Client::with_uri_str("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0").await;
        let database = client.unwrap().database("cli_chat");

        let mut vect = Vec::new();
        vect.push("val".to_string());

        //add group chat
        let add_group_chat_outcome =
            add_group_chat_w_db(database, "testname1".to_string(), "".to_string(), vect).await;

        assert_eq!(
            add_group_chat_outcome.unwrap(),
            AddGroupChatOutcome::CreatorEmailNotFound
        );
    }

    #[tokio::test]
    async fn test_add_group_chat_some_email_not_found() {
        //get database for test
        let client: Result<Client, mongodb::error::Error> = Client::with_uri_str("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0").await;
        let database = client.unwrap().database("cli_chat");

        let mut vect = Vec::new();
        vect.push("val".to_string());

        //add group chat
        let add_group_chat_outcome = add_group_chat_w_db(
            database,
            "testname1".to_string(),
            "test9@test.com".to_string(),
            vect,
        )
        .await;

        assert_eq!(
            add_group_chat_outcome.unwrap(),
            AddGroupChatOutcome::SomeEmailNotFound("val".to_string())
        );
    }

    #[tokio::test]
    async fn test_add_group_chat_not_friends() {
        //get database for test
        let client: Result<Client, mongodb::error::Error> = Client::with_uri_str("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0").await;
        let database = client.unwrap().database("cli_chat");

        let mut vect = Vec::new();
        vect.push("test11@test.com".to_string());
        vect.push("test1@test.com".to_string());

        //add group chat
        let add_group_chat_outcome = add_group_chat_w_db(
            database,
            "testname1".to_string(),
            "test9@test.com".to_string(),
            vect,
        )
        .await;

        assert_eq!(
            add_group_chat_outcome.unwrap(),
            AddGroupChatOutcome::SomeEmailNotFriends("test1@test.com".to_string())
        );
    }

    #[tokio::test]
    async fn test_add_group_chat_success() {
        // get database
        let client: Result<Client, mongodb::error::Error> = Client::with_uri_str("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0").await;
        let database = client.unwrap().database("cli_chat");
        let user_coll: Collection<Document> = database.collection("users");
        let group_chat_coll: Collection<Document> = database.collection("group_chats");

        // remove group chat
        let mut friends_user_docs: Vec<Document> = Vec::new();
        let mut friends_emails = Vec::new();
        friends_emails.push("test9@test.com".to_string());
        friends_emails.push("test10@test.com".to_string());
        friends_emails.push("test11@test.com".to_string());

        for friend_email in friends_emails.as_slice() {
            let get_doc_res = friends_routes::get_user_doc(&user_coll, friend_email).await;
            if !get_doc_res.is_err() {
                friends_user_docs.push(get_doc_res.unwrap())
            }
        }

        let mut group_chat_id: ObjectId = ObjectId::new();
        // remove groupchat ids
        for friend_doc in friends_user_docs {
            // Add groupchat ids
            let mut friend_group_chat_vec = get_group_chat_vec_from_doc(&friend_doc);
            group_chat_id = friend_group_chat_vec.pop().unwrap();

            // Update creator group chat list on mongodb
            let friend_update_doc = doc! {
                "$set": doc! { "group_chats": friend_group_chat_vec.as_slice()},
            };

            assert!(!(user_coll.update_one(friend_doc, friend_update_doc).await).is_err());
        }

        assert!(!group_chat_coll
            .find_one_and_delete(doc! {"_id": group_chat_id})
            .await
            .is_err());

        // add group chat
        let mut vect = Vec::new();
        vect.push("test11@test.com".to_string());
        vect.push("test10@test.com".to_string());

        let add_group_chat_outcome = add_group_chat_w_db(
            database,
            "testname1".to_string(),
            "test9@test.com".to_string(),
            vect,
        )
        .await;
        assert_eq!(
            add_group_chat_outcome.unwrap(),
            AddGroupChatOutcome::Success(group_chat_id)
        );
    }

    #[tokio::test]
    async fn test_message_to_group_chat() {
        // This should work
        let client: Result<Client, mongodb::error::Error> = Client::with_uri_str("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0").await;
        let database = client.unwrap().database("cli_chat");

        let message_coll: Collection<Document> = database.collection("messages");
        let filter = doc! { "author_email": "test12@test.com" };

        let result = message_coll.delete_one(filter).await;
        assert_eq!(result.unwrap().deleted_count, 1);

        let send_message_outcome = send_message_group_chat_w_db(
            database,
            "test12@test.com".to_string(),
            ObjectId::from_str("674b536a52a18844befce6d9").unwrap(),
            "hellooo".to_string(),
        )
        .await;

        assert_eq!(send_message_outcome.unwrap(), SendMessageOutcome::Success)
    }

    #[tokio::test]
    async fn test_retrieve_messages() {
        // ensure that it only gets the messages for the group chat btw. the two users
        let client: Result<Client, mongodb::error::Error> = Client::with_uri_str("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0").await;
        let database = client.unwrap().database("cli_chat");

        let messages_vec = get_messages_group_chat(
            database,
            ObjectId::from_str("674b536a52a18844befce6d9").unwrap(),
        )
        .await
        .unwrap()
        .unwrap();
        assert!(messages_vec.contains(&Message {
            sender: "test13@test.com".to_string(),
            date_string: "2024-11-30 18:32:52.255 +00:00:00".to_string(),
            content: "hiii".to_string()
        }))
    }

    #[tokio::test]
    async fn test_get_group_chat_ids_names_map() {
        let client: Result<Client, mongodb::error::Error> = Client::with_uri_str("mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0").await;
        let database = client.unwrap().database("cli_chat");

        let hash_map = get_group_chat_ids_names_map(database, "test13@test.com".to_string())
            .await
            .unwrap()
            .unwrap();
        assert!(hash_map.contains_key(&ObjectId::from_str("674b536a52a18844befce6d9").unwrap()));
        assert_eq!(
            hash_map
                .get(&ObjectId::from_str("674b536a52a18844befce6d9").unwrap())
                .unwrap(),
            &"testname2".to_string()
        );
        assert_eq!(hash_map.len(), 1)
    }
}
