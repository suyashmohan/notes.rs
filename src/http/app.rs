use actix_web::web;
use crate::http::index;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .service(web::scope("/user")
                .route("/", web::get().to_async(index::get))
                .route("/hello", web::get().to_async(index::get))
            )
            .service(web::scope("/notes")
                .route("/", web::get().to_async(index::get))
                .route("/test", web::get().to_async(index::get))
            )
    );
}