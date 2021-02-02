#![allow(non_snake_case)]

use actix_web::{HttpResponse, web};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use uuid::Uuid;

use crate::DBPool;
use crate::federation::schemas::NewPost;
use crate::internal::{LocalNewPost, NewUser};

use self::models::*;

pub mod actions;
pub mod models;
pub mod schema;

fn naive_date_time_now() -> NaiveDateTime {
    NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0)
}

// TODO: Refactor all other endpoints to use this!
pub fn get_conn_from_pool(
    pool: web::Data<DBPool>,
) -> actix_web::Result<PooledConnection<ConnectionManager<MysqlConnection>>> {
    pool.get()
        .map_err(|_| HttpResponse::ServiceUnavailable().finish().into())
}
