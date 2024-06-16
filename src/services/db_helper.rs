use mongodb::{
    bson::{doc, Document},
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};

use crate::CONFIG;

const REDDIT_POSTS_COLLECTION: &str = "reddit_posts";
const POST_ID: &str = "reddit_posts";

pub struct DbService {
    pub client: Client,
}

impl DbService {
    pub async fn new() -> DbService {
        let mut client_options =
            ClientOptions::parse(CONFIG.get().unwrap().mongodb_url.to_string())
                .await
                .unwrap();

        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);

        DbService {
            client: Client::with_options(client_options).unwrap(),
        }
    }

    pub async fn check_post_exists(&self, post_id: &str) -> bool {
        let database = self
            .client
            .database(&CONFIG.get().unwrap().mongodb_database.to_string());
        let collection = database.collection::<Document>(REDDIT_POSTS_COLLECTION);

        let filter = doc! { POST_ID: post_id };

        match collection.find_one(filter, None).await {
            Ok(Some(_)) => true,
            Ok(None) => false,
            Err(error) => {
                println!("Error: {:?}", error);
                true
            }
        }
    }

    pub async fn add_post(&self, post_id: &str) {
        let database = self
            .client
            .database(&CONFIG.get().unwrap().mongodb_database.to_string());
        let collection = database.collection::<Document>(REDDIT_POSTS_COLLECTION);

        let document = doc! { POST_ID: post_id };

        match collection.insert_one(document, None).await {
            Ok(_) => {
                println!("Post {} added successfully.", post_id);
            }
            Err(error) => {
                println!("Error adding post: {:?}", error);
            }
        }
    }
}
