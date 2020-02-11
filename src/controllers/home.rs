use actix_web::Responder;


pub async fn index(couter: u32) -> impl Responder {
   format!( "Goodbye Hello World {}", couter )
}
