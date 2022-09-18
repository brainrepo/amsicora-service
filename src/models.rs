use crate::schema::users;
use crate::schema::users::dsl::*;
use crate::utils::user_token::TokenPayload;
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::{insert_into, prelude::*};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginDTO {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginInfoDTO {
    pub email: String,
    pub id: String,
}

#[derive(Deserialize)]
pub struct RegisterDTO {
    pub email: String,
    pub password: String,
}

impl User {
    pub fn find_all(conn: &mut PgConnection) -> QueryResult<Vec<User>> {
        users.get_results::<User>(conn)
    }

    pub fn login(login: LoginDTO, conn: &mut PgConnection) -> Option<String> {
        if let Ok(user_to_verify) = users
            .filter(email.eq(&login.email))
            .get_result::<User>(conn)
        {
            if !user_to_verify.password.is_empty()
                && verify(&login.password, &user_to_verify.password).unwrap()
            {
                return Some(TokenPayload::generate_token(&LoginInfoDTO {
                    email: user_to_verify.email,
                    id: user_to_verify.id,
                }));
            }
            None
        } else {
            None
        }
    }

    pub fn register(user: RegisterDTO, conn: &mut PgConnection) -> Result<String, String> {
        let hashed_pwd = hash(&user.password, DEFAULT_COST).unwrap();
        let uuid = Uuid::new_v4().to_string();
        let user_to_insert = User {
            id: uuid,
            password: hashed_pwd,
            email: user.email,
        };

        match insert_into(users).values(&user_to_insert).execute(conn) {
            Ok(_) => Ok("utente inserito correttamente".to_string()),
            Err(_) => Err("non é stato possibile inserire l'utente".to_string()),
        }
    }
}
