use std::io;
use actix_web::{web, App, HttpServer, middleware};

use crate::http::notes;
use crate::DBPool;

pub fn run(address: &str, pool: DBPool) -> io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .data(pool.clone())
            .configure(config)
    })
    .bind(address)?
    .run()
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .service(web::scope("/notes")
                .route("", web::post().to_async(notes::create))
                .route("/{id}", web::get().to_async(notes::get))
                .route("/{id}", web::put().to_async(notes::update))
                .route("/{id}", web::delete().to_async(notes::delete))
            )
    );
}