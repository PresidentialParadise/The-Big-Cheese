use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Error,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Collection,
};

use crate::models::{Recipe, User};

pub struct RecipeRepository {
    recipes: Collection<Recipe>,
}

impl RecipeRepository {
    pub fn new(recipes: Collection<Recipe>) -> RecipeRepository {
        RecipeRepository { recipes }
    }
    pub async fn _create_recipe(&self, recipe: Recipe) -> Result<InsertOneResult, Error> {
        self.recipes.insert_one(recipe, None).await
    }

    pub async fn _read_recipe(&self, id: ObjectId) -> Result<mongodb::Cursor<Recipe>, Error> {
        self.recipes.find(doc! { "_id": id}, None).await
    }

    pub async fn _update_recipe(
        &self,
        id: ObjectId,
        recipe: Recipe,
    ) -> Result<UpdateResult, Error> {
        self.recipes
            .replace_one(doc! {"_id": id}, recipe, None)
            .await
    }

    pub async fn _delete_recipe(&self, id: ObjectId) -> Result<DeleteResult, Error> {
        self.recipes.delete_one(doc! { "_id": id }, None).await
    }
}

pub struct UserRepository {
    users: Collection<User>,
}

impl UserRepository {
    pub fn new(users: Collection<User>) -> UserRepository {
        UserRepository { users }
    }
    pub async fn _create_user(&self, user: User) -> Result<InsertOneResult, Error> {
        self.users.insert_one(user, None).await
    }

    pub async fn _read_user(&self, id: ObjectId) -> Result<mongodb::Cursor<User>, Error> {
        self.users.find(doc! { "_id": id}, None).await
    }

    pub async fn _update_user(&self, id: ObjectId, user: User) -> Result<UpdateResult, Error> {
        self.users.replace_one(doc! {"_id": id}, user, None).await
    }

    pub async fn _delete_user(&self, id: ObjectId) -> Result<DeleteResult, Error> {
        self.users.delete_one(doc! { "_id": id }, None).await
    }
}
