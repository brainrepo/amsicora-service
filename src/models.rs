use crate::schema::users::dsl::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn find_all(conn: &mut PgConnection) -> QueryResult<Vec<User>> {
        users.load::<User>(conn)
    }
}
