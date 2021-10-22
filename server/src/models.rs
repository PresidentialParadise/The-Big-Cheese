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
struct Recipe {
    // _id: Option<ObjectId>,
    title: String,
    description: String,
    servings: String,
    ingredients: Vec<Ingredient>,
    instructions: Vec<String>,
    tags: Vec<String>,
    categories: Vec<String>,
    prep_time: String,
    cook_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Ingredient {
    title: String,
    note: String,
    quantity: Quantity,
}

#[derive(Debug, Serialize, Deserialize)]
struct Quantity {
    value: String,
    unit: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    // ? alternative: recipe_ids: Vec<ObjectId>
    recipes: Vec<Recipe>,
}
