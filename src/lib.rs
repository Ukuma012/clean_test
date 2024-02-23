use actix_web::dev::Server;
use std::net::TcpListener;

extern crate dotenv;
extern crate log;

#[macro_use]
extern crate diesel;
extern crate r2d2;

pub fn run(listner: TcpListener, db_name: &str) -> Result<Server, std::io::Error> {}
