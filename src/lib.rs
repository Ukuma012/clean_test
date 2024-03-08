use actix_web::dev::Server;
use std::net::TcpListener;

pub mod adapters;
pub mod application;
pub mod domain;
pub mod error;
pub mod error_response;
pub mod infrastructure;

extern crate log;

#[macro_use]
extern crate diesel;
extern crate r2d2;

pub async fn run(listner: TcpListener, db_name: &str) -> Result<Server, std::io::Error> {
    infrastructure::server::server(listner, db_name).await
}
