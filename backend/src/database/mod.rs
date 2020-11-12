#![allow(non_snake_case)]

use crate::DBPool;
use actix_web::{web, HttpResponse};
use chrono::{NaiveDateTime, Utc};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::MysqlConnection;

pub mod federation;
pub mod local;
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
