use actix_web::get;
use actix_web::web;
use actix_web::Result;

#[get("/hello/{name}")]
pub async fn hello(web::Path(name): web::Path<String>) -> Result<String> {
    Ok(format!("Hello {}", name))
}
