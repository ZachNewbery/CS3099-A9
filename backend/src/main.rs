#[macro_use]
extern crate diesel;

use crate::federation::communities::{communities, community_by_id, community_by_id_timestamps};
use crate::federation::posts::{delete_post, edit_post, new_post_federated, post_by_id, posts};
use crate::federation::users::{search_users, send_user_message, user_by_id};
use crate::internal::{get_posts, login, logout, new_post_local, new_user};
use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

pub mod database;
pub mod federation;
pub mod internal;
pub mod util;

type DBPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Make sure dotenvs are all here
    dotenv::from_filename("setup.env").ok();
    dotenv::from_filename(".env").expect("no database source found");
    let bind = format!(
        "{}:{}",
        std::env::var("BIND_ADDRESS").expect("BIND_ADDRESS"),
        std::env::var("BIND_PORT").expect("BIND_PORT")
    );

    let manager = ConnectionManager::<MysqlConnection>::new(std::env::var("DATABASE_URL").unwrap());
    let pool = r2d2::Pool::builder()
        .max_size(8)
        .build(manager)
        .expect("could not set up threadpool for database");

    println!("Starting server on: {}", &bind);

    // Start the server!
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(pool.clone())
            .service(
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
                            .service(new_post_federated)
                            .service(post_by_id)
                            .service(edit_post)
                            .service(delete_post),
                    )
                    .service(
                        web::scope("/users")
                            .service(search_users)
                            .service(user_by_id)
                            .service(send_user_message),
                    ),
            )
            .service(
                web::scope("/internal")
                    .service(new_user)
                    .service(login)
                    .service(logout)
                    .service(new_post_local)
                    .service(get_posts),
            )
            .service(federation::hello)
            .service(federation::key)
            .service(federation::discover)
    })
    .bind(bind)?
    .run()
    .await
}
