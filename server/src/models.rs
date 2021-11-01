use mongodb::bson::oid::ObjectId;
// use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::time::Duration;

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
    pub hashed_password: String,
    pub admin: bool,

    pub recipes: Vec<ObjectId>,
    pub tokens: Vec<DatedToken>
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Token{
    pub token: Uuid,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct DatedToken {
    pub token: Token,
    pub date: DateTime<Utc>
}

impl DatedToken {
    pub fn generate() -> Self {
        DatedToken {
            token: Token{token: Uuid::new_v4()},
            date: Utc::now(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    expiration_time: Duration
}

impl Default for Config {
    fn default() -> Self {
        Self {
            expiration_time: chrono::Duration::hours(8).to_std().expect("in range"),
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Meta {
    config: Config
}