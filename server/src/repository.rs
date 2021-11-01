use mongodb::{
    bson::{doc, oid::ObjectId, to_bson},
    error::Error,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Collection,
};

use crate::auth::register;
use crate::error::CheeseError;
use crate::models::{Config, DatedToken, Meta, Recipe, Token, User};
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

    /// Creates a user in the database. Should usually be done using the [`register`] function
    /// (which calls this method).
    ///
    /// ```
    /// # use big_cheese_server::test_util::test_db;
    /// use big_cheese_server::models::User;
    /// # test_db(|client| async move {
    ///     let users = &client.user_repo;
    ///     let user = User {
    ///        id: None,
    ///        username: "test".into(),
    ///        display_name: "tester".into(),
    ///        hashed_password: "something".into(),
    ///        admin: false,
    ///        recipes: vec![],
    ///        tokens: vec![],
    ///    };
    ///
    ///     users.create_user(user.clone()).await.unwrap();
    ///
    ///     let mut stored_user = users.get_user_by_name(&user.username).await.unwrap().unwrap();
    ///     assert!(stored_user.id.is_some());
    ///     // remove to test equality with the original user
    ///     stored_user.id = None;
    ///     assert_eq!(user, stored_user);
    /// # });
    /// ```
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

    pub async fn get_user_for_token(&self, token: Token) -> Result<Option<User>, Error> {
        let d = to_bson(&token)?;
        self.collection
            .find_one(doc! { "tokens": {"$elemMatch": {"token": d}}}, None)
            .await
    }

    pub async fn remove_token(
        &self,
        user: &User,
        token: &DatedToken,
    ) -> Result<UpdateResult, Error> {
        let d = to_bson(token)?;
        self.collection
            .update_one(
                doc! { "username": &user.username },
                doc! {"$pull": {
                    "tokens": d
                }},
                None,
            )
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

    /// looks if a meta (and with it a config) is already in the database.
    /// If not, this method creates a default version.
    ///
    /// This method should be called whenever the server starts up. If not, [`get_config`] panics.
    ///
    /// ```should_panic
    /// # use big_cheese_server::test_util::test_db_panic;
    /// use std::panic::catch_unwind;
    /// # test_db_panic(|client| async move {
    ///     let meta = &client.meta_repo;
    ///     meta.get_config().await;
    /// # });
    /// ```
    ///
    /// ```
    /// # use big_cheese_server::test_util::test_db;
    /// # test_db(|client| async move {
    ///     let meta = &client.meta_repo;
    ///
    ///     meta.set_default_meta_if_not_exists().await.unwrap();
    ///
    ///     assert!(meta.get_config().await.is_ok());
    /// # });
    /// ```
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

    /// Gets the server wide config from the database. Make sure that this config is present.
    /// To ensure this, use [`set_default_meta_if_not_exists`].
    ///
    /// Examples of using this method can be found in the documentation of [`set_default_meta_if_not_exists`].
    pub async fn get_config(&self) -> Result<Config, Error> {
        let res = self
            .collection
            .find_one(doc! {}, None)
            .await?
            .expect("config to be set at startup");
        Ok(res.config)
    }
}
