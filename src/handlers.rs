use crate::models::{LoginDTO, RegisterDTO};

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

pub async fn login(
    login_dto: web::Json<LoginDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let login_response = web::block(move || User::login(login_dto.0, &mut pool.get().unwrap()))
        .await
        .unwrap();

    match login_response {
        Some(login_info_dto) => Ok(HttpResponse::Ok().json(login_info_dto)),
        None => Ok(HttpResponse::Unauthorized().body(("User not authorized to login").to_string())),
    }
}

pub async fn register(
    register_dto: web::Json<RegisterDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let register_response =
        web::block(move || User::register(register_dto.0, &mut pool.get().unwrap()))
            .await
            .unwrap();

    match register_response {
        Ok(_) => Ok(HttpResponse::Ok().body("user inserted correctly".to_string())),
        Err(_) => Ok(HttpResponse::InternalServerError()
            .body("Something went wrong while inserting the user".to_string())),
    }
}
