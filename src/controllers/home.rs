
use actix_web::{HttpResponse, Error};

#[warn(unused_must_use)]
#[allow(dead_code)]
pub async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hello index"))
}

#[warn(unused_must_use)]
#[allow(dead_code)]
pub async fn hello() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hello world"))
}
