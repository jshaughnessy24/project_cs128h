use mongodb::{ 
	bson::{Document, doc},
	Client,
	Collection 
};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    // Replace the placeholder with your Atlas connection string
    // let uri = "mongodb+srv://jennys4:3tA6Ui0z2MPrUnyk@cluster0.jwcji.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0";

    // // Create a new client and connect to the server
    // let client = Client::with_uri_str(uri).await?;

    // // Get a handle on the movies collection
    // let database = client.database("sample_mflix");
    // let my_coll: Collection<Document> = database.collection("movies");

    // // Find a movie based on the title value
    // let my_movie = my_coll.find_one(doc! { "title": "The Perils of Pauline" }).await?;

    // // Print the document
    // println!("Found a movie:\n{:#?}", my_movie);

    // project_cs128h::sign_in("".to_string(),"".to_string(),"".to_string()).await;
    
    Ok(())
}
