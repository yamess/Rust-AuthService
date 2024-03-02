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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_type_of() {
        let a = 1;
        let b = "hello";
        let c = 1.0;
        let d = vec![1, 2, 3];
        let e = (1, 2);
        let f = Some(1);
        let g: std::option::Option<i32> = None;
        let h = &a;
        let i = &b;

        assert_eq!(type_of(a), "i32");
        assert_eq!(type_of(b), "&str");
        assert_eq!(type_of(c), "f64");
        assert_eq!(type_of(d), "alloc::vec::Vec<i32>");
        assert_eq!(type_of(e), "(i32, i32)");
        assert_eq!(type_of(f), "core::option::Option<i32>");
        assert_eq!(type_of(g), "core::option::Option<i32>");
        assert_eq!(type_of(h), "&i32");
        assert_eq!(type_of(i), "&&str");
    }
}
