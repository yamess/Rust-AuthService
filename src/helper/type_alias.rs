use bb8::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

pub type DbPool = Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;
