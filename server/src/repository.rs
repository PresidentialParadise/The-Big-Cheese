use mongodb::{
    bson::{doc, oid::ObjectId, to_bson},
    error::Error,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Collection,
};

use crate::auth::register;
use crate::error::CheeseError;
use crate::models::{Config, Meta, Recipe, Token, User};
use futures::StreamExt;

#[derive(Clone)]
pub struct Recipes {
    collection: Collection<Recipe>,
}

impl Recipes {
    pub fn new(recipes: Collection<Recipe>) -> Self {
        Self {
            collection: recipes,
        }
    }
    pub async fn create_recipe(&self, recipe: Recipe) -> Result<InsertOneResult, Error> {
        self.collection.insert_one(recipe, None).await
    }

    pub async fn read_recipe(&self, id: ObjectId) -> Result<Option<Recipe>, Error> {
        self.collection.find_one(doc! { "_id": id}, None).await
    }

    pub async fn get_all_recipes(&self) -> Result<mongodb::Cursor<Recipe>, Error> {
        self.collection.find(None, None).await
    }

    pub async fn update_recipe(&self, id: ObjectId, recipe: Recipe) -> Result<UpdateResult, Error> {
        self.collection
            .replace_one(doc! {"_id": id}, recipe, None)
            .await
    }

    pub async fn delete_recipe(&self, id: ObjectId) -> Result<DeleteResult, Error> {
        self.collection.delete_one(doc! { "_id": id }, None).await
    }
}

#[derive(Clone)]
pub struct Users {
    collection: Collection<User>,
}

impl Users {
    pub fn new(users: Collection<User>) -> Self {
        Self { collection: users }
    }
    pub async fn create_user(&self, user: User) -> Result<InsertOneResult, Error> {
        self.collection.insert_one(user, None).await
    }

    pub async fn get_user_by_id(&self, id: ObjectId) -> Result<Option<User>, Error> {
        self.collection.find_one(doc! { "_id": id}, None).await
    }

    pub async fn get_all_users(&self) -> Result<mongodb::Cursor<User>, Error> {
        self.collection.find(None, None).await
    }

    pub async fn get_user_by_name(&self, name: &str) -> Result<Option<User>, Error> {
        self.collection
            .find_one(doc! { "username": name}, None)
            .await
    }

    pub async fn get_user_for_token(&self, t: Token) -> Result<Option<User>, Error> {
        let d = to_bson(&t).unwrap();
        self.collection
            .find_one(doc! { "tokens": {"$elemMatch": {"token": d}}}, None)
            .await
    }

    pub async fn update_user(&self, id: ObjectId, user: User) -> Result<UpdateResult, Error> {
        self.collection
            .replace_one(doc! {"_id": id}, user, None)
            .await
    }

    pub async fn delete_user(&self, id: ObjectId) -> Result<DeleteResult, Error> {
        self.collection.delete_one(doc! { "_id": id }, None).await
    }

    pub async fn first_user(&self, name: &str, password: &str) -> Result<(), CheeseError> {
        if self.get_all_users().await?.next().await.is_none() {
            register(self, name, password).await?;
            tracing::info!(
                "generated initial user with name '{}' and password '{}'",
                name,
                password
            );
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct MetaRepo {
    collection: Collection<Meta>,
}

impl MetaRepo {
    pub fn new(meta: Collection<Meta>) -> Self {
        Self { collection: meta }
    }

    pub async fn set_default_meta_if_not_exists(&self) -> Result<(), Error> {
        if self.collection.find_one(doc! {}, None).await?.is_none() {
            self.collection.insert_one(Meta::default(), None).await?;
        }

        Ok(())
    }

    #[allow(unused)]
    pub async fn set_config(&self, config: Config) -> Result<(), Error> {
        self.collection
            .find_one_and_update(doc! {}, doc! {"$set": {"config" : to_bson(&config)?}}, None)
            .await?;

        Ok(())
    }

    pub async fn get_config(&self) -> Result<Config, Error> {
        let res = self
            .collection
            .find_one(doc! {}, None)
            .await?
            .expect("config to be set at startup");
        Ok(res.config)
    }
}
