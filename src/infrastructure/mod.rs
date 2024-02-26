use std::{env, net::TcpListener};

use crate::adapters::{
    self,
    api::shared::app_state::AppState,
    spi::db::{db_connection::DbConnection, db_dog_facts_repository::DogFactsRepository, db_invitation_repository::InvitationRepository},
};

use actix_web::{dev::Server, middleware::Logger};
use actix_web::{web, App, HttpServer};

pub fn server(listner: TcpListener, db_name: &str) -> Result<Server, std::io::Error> {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=debug");

    env_logger::init();

    let db_connection = DbConnection { db_name: db_name.to_string() };

    let data = web::Data::new(AppState {
        app_name: String::from("Clean Architecture Test"),
        dogs_repository: DogFactsRepository {
            db_connection: db_connection.clone(),
        },
        invitation_repository: InvitationRepository {
            db_connection: db_connection.clone(),
        },
    });

    let port = listner.local_addr().unwrap().port();

    let server = HttpServer::new(move || App::new().app_data(data.clone()).wrap(Logger::default()).configure(adapters::api::shared::routes::routes))
        .listen(listner)?
        .run();

    println!("Server running on port {}, db_name {}", port, db_name);

    Ok(server)
}
