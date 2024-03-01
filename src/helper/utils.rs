use actix_web::web;
use bb8::PooledConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

use crate::helper::type_alias::DbPool;

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

// type of variable
pub fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}
