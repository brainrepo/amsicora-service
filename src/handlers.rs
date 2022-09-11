use super::models::User;

use super::Pool;
use actix_web::{web, Error, HttpResponse};
// pub async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("hello world")
// }

// #[post("/echo")]
// pub async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// pub async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there")
// }

// pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
//     Ok(web::block(move || get_all_users(db))
//         .await
//         .map(|user| HttpResponse::Ok().json(user))
//         .map_err(|_| HttpResponse::InternalServerError())?)
// }

// fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
//     let mut conn = pool.get().unwrap();
//     let items = users.load::<User>(&mut conn)?;
//     Ok(items)
// }

pub async fn find_all(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let users = web::block(move || {
        let mut conn = pool.get().unwrap();
        User::find_all(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError);

    match users {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        Err(_) => Ok(HttpResponse::NotFound().body(("No user found").to_string())),
    }
}
