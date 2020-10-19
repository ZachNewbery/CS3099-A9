use actix_web::{middleware, web, App, HttpServer};

pub mod federation;

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
            .service(web::scope("/federation").service(federation::hello))
    })
    .bind(bind)?
    .run()
    .await
}
