use std::fmt::Debug;

use diesel::result::Error;
use diesel_async::AsyncPgConnection;
use serde::{Deserialize, Serialize};

pub trait IRepository<'a, T, R>
where
    T: Debug + Serialize + Deserialize<'a>,
    R: Debug + Serialize + Deserialize<'a>,
{
    type Model;
    async fn create(conn: &mut AsyncPgConnection, data: T) -> Result<R, Error>;
    // async fn get(conn: &mut AsyncPgConnection, id: &Identifier) -> Result<Option<R>, RepositoryError>;
    // async fn update(conn: &mut AsyncPgConnection, id: &Identifier, new_data: U) -> Result<R, RepositoryError>;
    // async fn delete(conn: &mut AsyncPgConnection, id: &Identifier) -> Result<usize, RepositoryError>;
}
