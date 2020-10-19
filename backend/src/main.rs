use actix_web::{get, middleware, web, App, HttpServer, Result};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let bind = format!(
        "{}:{}",
        std::env::var("BIND_ADDRESS").expect("BIND_ADDRESS"),
        std::env::var("BIND_PORT").expect("BIND_PORT")
    );

    println!("Starting server on: {}", &bind);

    // Start the server!
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(hello)
    })
    .bind(bind)?
    .run()
    .await
}

#[get("/hello/{name}")]
async fn hello(web::Path(name): web::Path<String>) -> Result<String> {
    Ok(format!("Hello {}", name))
}
