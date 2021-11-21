#[macro_use]
extern crate lazy_static;

use actix_web::{middleware, App, HttpServer};
use std::{env, thread};

mod queue;
mod route;
mod util;
mod ws;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    thread::spawn(ws::taskloop);
    let addr = opt();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().header("access-control-allow-origin", "*"))
            .service(route::hello)
            .service(route::status)
            .service(route::ws)
            .service(route::error_log)
    })
    .bind(addr)?
    .run()
    .await
}

fn opt() -> String {
    let mut opts: Vec<String> = vec![env::var("ADDR").unwrap_or("127.0.0.1:8080".to_string())];
    let mut index = 0;
    let mut first = true;
    for argument in env::args() {
        if first {
            first = false;
            continue;
        }
        if index >= 1 {
            break;
        }
        opts[index] = argument;
        index = index + 1;
    }
    opts[0].clone()
}
