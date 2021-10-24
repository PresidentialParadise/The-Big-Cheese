use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Error,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Collection,
};

use crate::models::{Recipe, User};

#[derive(Clone)]
pub struct Recipes {
    collection: Collection<Recipe>,
}

impl Recipes {
    pub fn new(recipes: Collection<Recipe>) -> Recipes {
        Recipes {
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
    pub fn new(users: Collection<User>) -> Users {
        Users { collection: users }
    }
    pub async fn create_user(&self, user: User) -> Result<InsertOneResult, Error> {
        self.collection.insert_one(user, None).await
    }

    pub async fn read_user(&self, id: ObjectId) -> Result<Option<User>, Error> {
        self.collection.find_one(doc! { "_id": id}, None).await
    }

    pub async fn get_all_users(&self) -> Result<mongodb::Cursor<User>, Error> {
        self.collection.find(None, None).await
    }

    pub async fn update_user(&self, id: ObjectId, user: User) -> Result<UpdateResult, Error> {
        self.collection
            .replace_one(doc! {"_id": id}, user, None)
            .await
    }

    pub async fn delete_user(&self, id: ObjectId) -> Result<DeleteResult, Error> {
        self.collection.delete_one(doc! { "_id": id }, None).await
    }
}
