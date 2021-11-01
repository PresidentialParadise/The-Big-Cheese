use crate::db_connection::DBClient;
use uuid::Uuid;
use tokio::runtime::Runtime;
use std::env;
use std::future::Future;
use std::panic::resume_unwind;

/// Use this function to temporarily crate a mongodb database
/// given the env var `TEST_DB_URI` or `DB_URI` (preferring `TEST_DB_URI`).
/// If no env var is given, the test automatically succeeds. When the test panics,
/// the database is still deleted after the test leaving no garbage behind.
///
/// To ensure `should_panic` tests pass when no database URI is provided, call [`test_db_panic`]
#[allow(unused)]
pub fn test_db<F, Res>(f: F) -> bool
    where
        F: 'static + Send + FnOnce(DBClient) -> Res,
        Res: Future<Output=()> + Send,
{
    let _ignored = dotenv::dotenv();

    let name = format!("test_db_{}", Uuid::new_v4());

    let client_uri = if let Ok(client_uri) = env::var("TEST_DB_URI")
        .or_else(|_| {
            env::var("DB_URI")
        }) {
        client_uri
    } else {
        return false;
    };

    let rt = Runtime::new().expect("create runtime");
    rt.block_on(async move {
        let client = DBClient::new(client_uri, name).await
            .expect("start mongodb client");


        let local_client = client.clone();
        let res = tokio::spawn(async move {
            f(local_client).await
        }).await;

        client.delete_db().await;

        if let Err(e) = res {
            if e.is_panic() {
                // Resume the panic on the main task
                resume_unwind(e.into_panic());
            }
        }
    });

    true
}

/// Panics when tests could not be run due to the lack of db uri. This is to
/// make sure should_panic tests stay working without a uri.
#[allow(unused)]
pub fn test_db_panic<F, Res>(f: F)
    where
        F: 'static + Send + FnOnce(DBClient) -> Res,
        Res: Future<Output=()> + Send,
{
    if !test_db(f) {
        panic!("no db found but test should panic");
    }
}

#[cfg(test)]
mod tests {
    use crate::test_util::test_db_panic;

    #[test]
    #[should_panic]
    fn test_panic() {
        test_db_panic(|_| async {
            panic!("test");
        });
    }
}