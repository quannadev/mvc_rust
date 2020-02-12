
use actix_web::{HttpResponse, Error};

pub async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hello index"))
}


pub async fn hello() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hello world"))
}