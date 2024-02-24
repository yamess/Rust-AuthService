use std::fmt::Debug;

use diesel::result::Error;
use diesel_async::AsyncPgConnection;
use serde::{Deserialize, Serialize};

use crate::helper::enums::Identifier;

pub trait IRepository<'a, T, U, R>
    where
        T: Debug + Serialize + Deserialize<'a>,
        U: Debug + Serialize + Deserialize<'a>,
        R: Debug + Serialize + Deserialize<'a>,
{
    type Model;
    async fn create(conn: &mut AsyncPgConnection, data: T) -> Result<R, Error>;
    async fn get(conn: &mut AsyncPgConnection, id: &Identifier) -> Result<Option<R>, Error>;
    async fn update(conn: &mut AsyncPgConnection, id: &Identifier, new_data: U) -> Result<R, Error>;
    // async fn delete(conn: &mut AsyncPgConnection, id: &Identifier) -> Result<usize, RepositoryError>;
}
