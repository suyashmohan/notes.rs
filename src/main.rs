use actix_web::{middleware, App, HttpServer};
use std::{env, io};

mod http;
use http::app;

fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();    
    
    let address = "127.0.0.1:8080";

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath)
            .configure(app::config)
    })
    .bind(address)?
    .run()
}
