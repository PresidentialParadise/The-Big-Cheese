use mongodb::bson::oid::ObjectId;
// use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum Measurement {
    /// Grams
    #[serde(rename = "kg")]
    Kilogram,
    #[serde(rename = "g")]
    Gram,
    #[serde(rename = "mg")]
    Milligram,
    /// Litres
    #[serde(rename = "l")]
    Litre,
    #[serde(rename = "dl")]
    Decilitre,
    #[serde(rename = "ml")]
    Millilitre,
    /// Spoons
    #[serde(rename = "tbs")]
    Tablespoon,
    #[serde(rename = "tsp")]
    Teaspoon,
    /// "Measurements"
    #[serde(rename = "gl")]
    Gallon,
    #[serde(rename = "qt")]
    Quart,
    #[serde(rename = "pt")]
    Pint,
    #[serde(rename = "cup")]
    Cup,
    /// Yeet
    Custom(String),
}

#[derive(Debug, Serialize, Deserialize)]
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
    pub prep_time: String,
    pub cook_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ingredient {
    title: String,
    note: String,
    quantity: Quantity,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quantity {
    value: String,
    unit: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    name: String,
    // ? alternative: recipe_ids: Vec<ObjectId>
    recipes: Vec<Recipe>,
}
