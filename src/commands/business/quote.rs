use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
use futures::stream::TryStreamExt;
use crate::utils::mongo::get_mongo_client;
use crate::utils::string_utils::capitalize;

pub const QUOTE_DB_NAME: &str = "Quotes";
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Quote {
    pub quote: String,
    pub year: String,
    pub author: String,
    pub context: String,
}
pub async fn get_quote_from(author_name: &String) -> Vec<Quote> {
    return get_quotes(doc! { "author": capitalize(author_name) }).await
}

pub async fn get_random_quote() -> Vec<Quote> {
    return get_quotes(doc! {  }).await;
}


pub async fn get_quotes(filter: Document) -> Vec<Quote> {
    let database = match get_mongo_client().await {
        Ok(db) => db,
        Err(err) => {
            log::error!("An error occurred while trying to get the db: {}", err);
            return vec![];
        }
    };

    let typed_collection = database.collection::<Quote>(QUOTE_DB_NAME);
    let cursor = match typed_collection.find(filter, None).await {
        Ok(quote_cursor) => quote_cursor,
        Err(err) => {
            log::error!("An error occurred while trying to find the quote: {}", err);
            return vec![];
        }
    };

    return cursor.try_collect().await.unwrap_or_else(|_| vec![]);
}

pub async fn add_quote_to_collection(quote_to_add: Quote) {
    let database = match get_mongo_client().await {
        Ok(db) => db,
        Err(err) => {
            log::error!("An error occurred while trying to get the db: {}", err);
            return;
        }
    };

    match database.collection::<Quote>(QUOTE_DB_NAME).insert_one(quote_to_add.clone(), None).await.ok() {
        Some(result) => {
            log::info!("Added quote [{}] to the DB: id: {}", quote_to_add.quote.clone(), result.inserted_id)
        }
        None => {
            log::error!("Something went wrong trying to add the quote []")
        }
    }
}