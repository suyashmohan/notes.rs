use std::io;
use actix_web::{web, App, HttpServer, middleware};

use crate::http::index;
use crate::http::notes;
use crate::Pool;

pub fn run(address: &str, pool: Pool) -> io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath)
            .data(pool.clone())
            .configure(config)
    })
    .bind(address)?
    .run()
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .service(web::scope("/user")
                .route("/", web::get().to_async(index::get))
                .route("/hello", web::get().to_async(index::get))
            )
            .service(web::scope("/notes")
                .route("/", web::get().to_async(notes::create))
                .route("/test", web::get().to_async(index::get))
            )
    );
}