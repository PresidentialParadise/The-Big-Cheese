use std::str::FromStr;

use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    headers::{authorization::Bearer, Authorization},
    Extension, TypedHeader,
};
use jsonwebtoken::{decode, Validation};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::{
    auth::{jwt, Redirect, KEYS},
    db::DBClient,
    error::CheeseError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Measurement {
    /// Grams
    Kilogram,
    Gram,
    Milligram,
    /// Litres
    Litre,
    Decilitre,
    Millilitre,
    /// Spoons
    Tablespoon,
    Teaspoon,
    /// "Measurements"
    Gallon,
    Quart,
    Pint,
    Cup,
    /// Yeet
    Custom(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Recipe {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub description: String,
    pub servings: String,
    pub ingredients: Vec<Ingredient>,
    pub instructions: Vec<String>,
    pub tags: Vec<String>,
    pub categories: Vec<String>,
    pub prep_time: usize, // in minutes
    pub cook_time: usize, // in minutes
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ingredient {
    pub title: String,
    pub note: String,
    pub quantity: Option<Quantity>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Quantity {
    pub value: u8,
    pub unit: Measurement,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserList {
    pub users: Vec<User>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub display_name: String,
    pub password: String,
    pub admin: bool,
    pub recipes: Vec<ObjectId>,
}

#[derive(Debug, Deserialize)]
pub struct Details {
    pub username: String,
    pub display_name: String,
    pub password: String,
}

#[async_trait]
impl<B> FromRequest<B> for Details
where
    B: Send,
{
    type Rejection = CheeseError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| CheeseError::InvalidToken)?;

        let Extension(db_client) = Extension::<DBClient>::from_request(req)
            .await
            .map_err(|_| CheeseError::InvalidToken)?;

        let claims = jwt::verify(bearer.token())?;
        Ok(Details {
            username: "".to_string(),
            display_name: "".to_string(),
            password: "".to_string(),
        })
    }
}
