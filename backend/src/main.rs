#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

use internal::user::{get_user, login, logout, new_user};

use crate::federation::communities::{communities, community_by_id, community_by_id_timestamps};
use crate::federation::posts::{
    delete_post, edit_post, get_post_by_id, new_post_federated, post_matching_filters,
};

use crate::federation::users::{search_users, send_user_message, user_by_id};
use crate::internal::discover;
use crate::internal::posts::{create_post, get_post, list_posts, search_posts};
use crate::internal::user::edit_profile;

pub mod database;
pub mod federation;
pub mod internal;
pub mod util;

#[allow(clippy::upper_case_acronyms)]
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

        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().ends_with(b".rust-lang.org")
            })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(Cors::permissive())
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
                            .service(post_matching_filters)
                            .service(new_post_federated)
                            .service(get_post_by_id)
                            .service(edit_post)
                            .service(delete_post),
                    )
                    .service(
                        web::scope("/users")
                            .service(search_users)
                            .service(user_by_id)
                            .service(send_user_message),
                    )
                    .service(federation::key)
                    .service(federation::hello)
                    .service(federation::discover),
            )
            .service(
                web::scope("/internal")
                    .service(new_user)
                    .service(login)
                    .service(logout)
                    .service(edit_profile)
                    .service(get_user)
                    .service(get_post)
                    .service(list_posts)
                    .service(search_posts)
                    .service(create_post)
                    .service(discover)
                    .service(internal::posts::edit_post)
                    .service(internal::posts::delete_post)
                    .service(internal::communities::list_communities)
                    .service(internal::communities::create_community)
                    .service(internal::communities::get_community_details)
                    .service(internal::communities::delete_community)
                    .service(internal::communities::edit_community_details),
            )
    })
    .workers(2)
    .bind(bind)?
    .run()
    .await
}
