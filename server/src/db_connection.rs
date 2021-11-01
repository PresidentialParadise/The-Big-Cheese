use mongodb::{error::Error, Client, Database};

use crate::repository;

#[derive(Clone)]
pub struct DBClient {
    client: Client,
    database: Database,
    pub recipe_repo: repository::Recipes,
    pub user_repo: repository::Users,
    pub meta_repo: repository::MetaRepo,
}

impl DBClient {
    pub async fn new(client_uri: impl AsRef<str>, db_name: impl AsRef<str>) -> Result<Self, Error> {
        let client = Client::with_uri_str(client_uri).await?;
        let database = client.database(db_name.as_ref());
        let recipe_repo = repository::Recipes::new(database.collection("recipes"));
        let user_repo = repository::Users::new(database.collection("users"));
        let meta_repo = repository::MetaRepo::new(database.collection("meta"));

        Ok(Self {
            client,
            database,
            recipe_repo,
            user_repo,
            meta_repo,
        })
    }

    pub async fn delete_db(&self) {
        self.database.drop(None).await.expect("couldn't drop db")
    }
}
