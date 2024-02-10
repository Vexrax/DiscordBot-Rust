use std::env;

use mongodb::{options::ClientOptions, Client, Database};

const DB_NAME: &str = "Skynet";
pub async fn get_mongo_client() -> mongodb::error::Result<Database> {
    let mongo_pass = env::var("MONGOPASSWORD").expect("Expected MONGOPASSWORD in environment");
    let mongo_connection_string = format!("mongodb+srv://Skynet:{}@cluster0-mzmgc.mongodb.net/test?retryWrites=true&w=majority", mongo_pass);
    let client_options = ClientOptions::parse(mongo_connection_string).await?;
    let client = Client::with_options(client_options)?;
    let database = client.database(DB_NAME);
    Ok(database)
}