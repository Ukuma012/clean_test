use std::{env, net::TcpListener};

use crate::adapters::{
    self,
    api::shared::app_state::AppState,
    spi::{
        db::{
            db_connection::DbConnection, db_dog_facts_repository::DogFactsRepository, db_invitation_repository::InvitationRepository,
            db_register_complete_repository::RegisterCompleteRepository,
        },
        email::email_repository::MailTrapRepository,
    },
};

use actix_redis::RedisSession;
use actix_web::{dev::Server, middleware::Logger};
use actix_web::{web, App, HttpServer};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

pub fn server(listner: TcpListener, db_name: &str) -> Result<Server, std::io::Error> {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=debug,actix_redis=info");

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
        register_complete_repository: RegisterCompleteRepository {
            db_connection: db_connection.clone(),
        },
        email_repository: MailTrapRepository {},
    });

    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let mut csp_rng = ChaCha20Rng::from_entropy();
    let mut redis_data = [0u8; 32];
    csp_rng.fill_bytes(&mut redis_data);

    let port = listner.local_addr().unwrap().port();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(Logger::default())
            .wrap(RedisSession::new(&redis_url, &redis_data))
            .configure(adapters::api::shared::routes::routes)
    })
    .listen(listner)?
    .run();

    println!("Server running on port {}, db_name {}", port, db_name);

    Ok(server)
}
