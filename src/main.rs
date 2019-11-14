#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::{env, io};

mod data;
mod http;
mod model;
mod service;
mod schema;

use http::api;

pub(crate) type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn setup_db(address: &str) -> DBPool{
    let manager = ConnectionManager::<PgConnection>::new(address);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Unable to create Postgres Pool")
}

fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();    
    
    let address = "127.0.0.1:8080";
    let database_url = "postgres://postgres@localhost/rnote";

    api::run(address, setup_db(database_url))
}
