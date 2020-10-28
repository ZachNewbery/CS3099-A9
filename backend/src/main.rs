use crate::federation::communities::{communities, community_by_id, community_by_id_timestamps};
use crate::federation::posts::{delete_post, edit_post, new_post, post_by_id, posts};
use actix_web::{middleware, web, App, HttpServer};

pub mod database;
pub mod federation;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename("setup.env").ok();
    dotenv::from_filename(".env").expect("no database source found");
    let bind = format!(
        "{}:{}",
        std::env::var("BIND_ADDRESS").expect("BIND_ADDRESS"),
        std::env::var("BIND_PORT").expect("BIND_PORT")
    );

    println!("Starting server on: {}", &bind);
    // let connection = establish_connection();

    // Start the server!
    HttpServer::new(|| {
        App::new().wrap(middleware::Logger::default()).service(
            web::scope("/federation")
                .service(
                    web::scope("/communities")
                        .service(communities)
                        .service(community_by_id)
                        .service(community_by_id_timestamps),
                )
                .service(
                    web::scope("/posts")
                        .service(posts)
                        .service(new_post)
                        .service(post_by_id)
                        .service(edit_post)
                        .service(delete_post),
                )
                .service(federation::hello), // Hello!
        )
    })
    .bind(bind)?
    .run()
    .await
}
