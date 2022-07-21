use axum::{
    response::{self, IntoResponse, Response},
    Json,
};
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header};
use mongodb::results::InsertOneResult;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::{
    db::DBClient,
    error::{CheeseError, CheeseResult},
    models::{Details, User},
};

pub mod jwt;

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

struct Keys {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

#[derive(Debug, Serialize)]
pub struct Body {
    pub access_token: String,
    pub token_type: String,
}

impl Body {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Payload {
    pub client_id: String,
    pub client_secret: String,
}

pub struct Redirect;

impl IntoResponse for Redirect {
    fn into_response(self) -> Response {
        response::Redirect::temporary("/login").into_response()
    }
}

pub async fn authorise(Json(payload): Json<Payload>) -> CheeseResult<Json<Body>> {
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(CheeseError::MissingCredentials);
    }

    if payload.client_id != "" || payload.client_secret != "" {
        return Err(CheeseError::InvalidCredentials);
    }

    let claims = Claims {
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        // Mandatory expiry time as UTC timestamp
        exp: 2_000_000_000, // May 2033
    };

    let token = encode(
        &Header::default(),
        &claims,
        &Keys::new("hi".as_bytes()).encoding_key,
    )
    .map_err(|_| CheeseError::TokenCreation)?;

    Ok(Json(Body::new(token)))
}

pub async fn register(details: Details, db_client: &DBClient) -> CheeseResult<InsertOneResult> {
    if db_client
        .user_repo
        .find_user_by_name(&details.username)
        .await
        .is_ok()
    {
        return Err(CheeseError::UsernameExists);
    }

    let hash = bcrypt::hash(&details.password, 10)?;
    let user = User {
        id: None,
        username: details.username,
        display_name: details.display_name,
        password: hash,
        admin: false,
        recipes: vec![],
    };

    Ok(db_client.user_repo.create_user(user).await?)
}
