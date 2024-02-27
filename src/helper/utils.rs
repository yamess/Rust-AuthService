use actix_web::web;
use bb8::PooledConnection;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;

use crate::helper::type_alias::DbPool;

pub fn hash_password(password: &str) -> String {
    bcrypt::hash(password, 12).unwrap()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash).unwrap()
}

pub fn is_password_valid(password: &str) -> bool {
    password.len() >= 8 && password != "password" && password != "12345678"
}

pub async fn get_connection(
    pool: &web::Data<DbPool>,
) -> PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>> {
    pool.get()
        .await
        .map_err(|e| {
            log::error!("Failed to get pool: {}", e);
            actix_web::error::ErrorInternalServerError(e)
        })
        .unwrap()
}
