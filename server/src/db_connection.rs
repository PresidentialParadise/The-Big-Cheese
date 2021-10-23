use mongodb::{error::Error, Client, Database};

use crate::repository;

#[derive(Clone)]
pub struct DBClient {
    client: Client,
    database: Database,
    pub recipe_repo: repository::Recipes,
    pub user_repo: repository::Users,
}

impl DBClient {
    pub async fn new(client_uri: impl AsRef<str>, db_name: impl AsRef<str>) -> Result<Self, Error> {
        let client = Client::with_uri_str(client_uri).await?;
        let database = client.database(db_name.as_ref());
        let recipe_repo = repository::Recipes::new(database.collection("recipes"));
        let user_repo = repository::Users::new(database.collection("users"));

        Ok(Self {
            client,
            database,
            recipe_repo,
            user_repo,
        })
    }
}
