use std::str::FromStr;

use axum::{
    extract::{Extension, Path},
    Json,
};
use futures::TryStreamExt;
use mongodb::{
    bson::oid::ObjectId,
    results::{DeleteResult, UpdateResult},
};
use serde::{Deserialize, Serialize};

use crate::auth::middleware::{AdminAuth, AuthorizationError, SelfOrAdminAuth};
use crate::models::Token;
use crate::{
    auth,
    db_connection::DBClient,
    error::CheeseError,
    models::{User, UserList},
};

pub async fn fetch_users(
    Extension(db_client): Extension<DBClient>,
    _auth: AdminAuth,
) -> Result<Json<UserList>, CheeseError> {
    let cursor = db_client.user_repo.get_all_users().await?;

    let mut users: Vec<User> = cursor.try_collect().await?;
    for u in &mut users {
        u.hashed_password = String::new();
    }

    Ok(Json(UserList { users }))
}

pub async fn fetch_user(
    Path(id): Path<String>,
    Extension(db_client): Extension<DBClient>,
    auth: SelfOrAdminAuth,
) -> Result<Json<Option<User>>, CheeseError> {
    let id = ObjectId::from_str(&id)?;

    auth.user_by_id(&id)?;

    let mut res = db_client.user_repo.get_user_by_id(id).await?;

    if let Some(ref mut user) = res {
        user.hashed_password = String::new();
    }

    Ok(Json(res))
}

pub async fn update_user(
    Path(id): Path<String>,
    Json(user): Json<User>,
    Extension(db_client): Extension<DBClient>,
    auth: SelfOrAdminAuth,
) -> Result<Json<UpdateResult>, CheeseError> {
    let id = ObjectId::from_str(&id)?;

    let req_user = auth.user_by_id(&id)?;

    // if the requesting user is not an admin, they can not set themselves to be admin
    if user.admin && !req_user.admin {
        return Err(AuthorizationError::NotAdmin.into());
    }

    // if the requesting user is different from the id in the body, error
    if user.id != req_user.id {
        return Err(AuthorizationError::NotSelf.into());
    }

    let res = db_client.user_repo.update_user(id, user).await?;

    Ok(Json(res))
}

pub async fn delete_user(
    Path(id): Path<String>,
    Extension(db_client): Extension<DBClient>,
    auth: SelfOrAdminAuth,
) -> Result<Json<DeleteResult>, CheeseError> {
    let id = ObjectId::from_str(&id)?;

    auth.user_by_id(&id)?;

    let res = db_client.user_repo.delete_user(id).await?;
    Ok(Json(res))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiUser {
    username: String,
    password: String,
}

pub async fn register(
    Json(user): Json<ApiUser>,
    Extension(db_client): Extension<DBClient>,
) -> Result<Json<Token>, CheeseError> {
    auth::register(&db_client.user_repo, &user.username, &user.password).await?;

    let token = auth::login(&db_client.user_repo, &user.username, &user.password).await?;
    Ok(Json(token))
}

pub async fn login(
    Json(user): Json<ApiUser>,
    Extension(db_client): Extension<DBClient>,
) -> Result<Json<Token>, CheeseError> {
    let token = auth::login(&db_client.user_repo, &user.username, &user.password).await?;
    Ok(Json(token))
}

#[cfg(test)]
mod tests {
    use crate::auth::middleware::{AdminAuth, SelfOrAdminAuth};
    use crate::handlers::{fetch_user, fetch_users, update_user};
    use crate::models::User;
    use crate::test_util::test_db;
    use axum::extract::{Extension, Path};
    use axum::Json;
    use mongodb::bson::oid::ObjectId;

    #[test]
    pub fn get_user_sanitized() {
        test_db(|client| async {
            let id = ObjectId::new();

            let user = User {
                id: Some(id),
                username: "a".to_string(),
                display_name: "b".to_string(),
                hashed_password: "c".to_string(),
                admin: false,
                recipes: vec![],
                tokens: vec![],
            };

            client.user_repo.create_user(user.clone()).await.unwrap();

            let res = fetch_user(
                Path(id.to_string()),
                Extension(client),
                SelfOrAdminAuth::new_for_test(user.clone()),
            )
            .await
            .unwrap()
            .0
            .unwrap();

            assert_eq!(&res.id, &Some(id));
            assert_eq!(&res.username, &user.username);

            // blanked out
            assert_eq!(res.hashed_password, "")
        });
    }

    #[test]
    pub fn get_users_sanitized() {
        test_db(|client| async {
            let user1 = User {
                id: Some(ObjectId::new()),
                username: "a".to_string(),
                display_name: "b".to_string(),
                hashed_password: "c".to_string(),
                admin: false,
                recipes: vec![],
                tokens: vec![],
            };

            let user2 = User {
                id: Some(ObjectId::new()),
                username: "a".to_string(),
                display_name: "b".to_string(),
                hashed_password: "c".to_string(),
                admin: true,
                recipes: vec![],
                tokens: vec![],
            };

            client.user_repo.create_user(user1.clone()).await.unwrap();
            client.user_repo.create_user(user2.clone()).await.unwrap();

            let res = fetch_users(Extension(client), AdminAuth(user2.clone()))
                .await
                .unwrap()
                .0
                .users;

            for i in res {
                assert_eq!(i.hashed_password, "");
            }
        });
    }

    #[test]
    pub fn make_self_admin() {
        test_db(|client| async {
            let id = ObjectId::new();

            let user = User {
                id: Some(id),
                username: "a".to_string(),
                display_name: "b".to_string(),
                hashed_password: "c".to_string(),
                admin: false,
                recipes: vec![],
                tokens: vec![],
            };

            let mut admin_user = user.clone();
            admin_user.admin = true;

            client.user_repo.create_user(user.clone()).await.unwrap();

            assert!(update_user(
                Path(id.to_string()),
                Json(admin_user),
                Extension(client),
                SelfOrAdminAuth::new_for_test(user.clone())
            )
            .await
            .is_err())
        });
    }

    #[test]
    pub fn make_other_admin() {
        test_db(|client| async {
            let id1 = ObjectId::new();
            let id2 = ObjectId::new();

            let user1 = User {
                id: Some(id1),
                username: "a".to_string(),
                display_name: "b".to_string(),
                hashed_password: "c".to_string(),
                admin: true,
                recipes: vec![],
                tokens: vec![],
            };

            let user2 = User {
                id: Some(id2),
                username: "a".to_string(),
                display_name: "b".to_string(),
                hashed_password: "c".to_string(),
                admin: true,
                recipes: vec![],
                tokens: vec![],
            };

            let mut admin_user = user2.clone();
            admin_user.admin = true;

            client.user_repo.create_user(user1.clone()).await.unwrap();
            client.user_repo.create_user(user2.clone()).await.unwrap();

            assert!(update_user(
                Path(id2.to_string()),
                Json(admin_user),
                Extension(client),
                SelfOrAdminAuth::new_for_test(user2.clone())
            )
            .await
            .is_ok())
        });
    }

    #[test]
    pub fn wrong_id_in_path() {
        test_db(|client| async {
            let id = ObjectId::new();

            let user = User {
                id: Some(id),
                username: "a".to_string(),
                display_name: "b".to_string(),
                hashed_password: "c".to_string(),
                admin: false,
                recipes: vec![],
                tokens: vec![],
            };

            let mut req_user = user.clone();
            req_user.id = Some(ObjectId::new());

            client.user_repo.create_user(user.clone()).await.unwrap();

            assert!(update_user(
                Path(id.to_string()),
                Json(req_user),
                Extension(client),
                SelfOrAdminAuth::new_for_test(user.clone())
            )
            .await
            .is_err())
        });
    }
}
