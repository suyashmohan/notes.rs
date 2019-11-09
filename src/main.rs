use std::{env, io};

mod http;
use http::api;

fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();    
    
    let address = "127.0.0.1:8080";
    api::run(address)
}
