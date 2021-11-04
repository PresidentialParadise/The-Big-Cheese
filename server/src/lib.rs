#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::unused_async,
    clippy::missing_errors_doc,
    clippy::must_use_candidate
)]

pub mod auth;
pub mod db_connection;
pub mod error;
pub mod handlers;
pub mod models;
pub mod repository;

pub mod test_util;
