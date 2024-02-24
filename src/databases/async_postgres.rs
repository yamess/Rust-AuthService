use bb8::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

use crate::configs::common::DatabaseConfig;

pub struct AsyncPostgresPool {
    pub pool: Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
}

impl AsyncPostgresPool {
    pub async fn new(database_config: &DatabaseConfig) -> Self {
        let database_url = &database_config.database_url;
        let pool_size = &database_config.max_pool_size;
        let db_manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
        let pool = Pool::builder()
            .max_size(*pool_size)
            .test_on_check_out(true)
            .build(db_manager)
            .await
            .expect("Failed to create pool");
        log::info!("Postgres connection pool created");
        AsyncPostgresPool { pool }
    }
}
