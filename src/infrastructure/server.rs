use std::{env, net::TcpListener};

use crate::adapters::{
    self,
    api::shared::app_state::AppState,
    spi::{
        db::{
            db_connection::DbConnection, db_dog_facts_repository::DogFactsRepository, db_invitation_repository::InvitationRepository, db_login_repository::LoginRepository,
            db_register_complete_repository::RegisterCompleteRepository,
        },
        email::email_repository::MailTrapRepository,
    },
};

use actix_session::{storage::RedisActorSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, web, App, HttpServer};
use actix_web::{dev::Server, middleware::Logger};

pub async fn server(listner: TcpListener, db_name: &str) -> Result<Server, std::io::Error> {
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
        login_repository: LoginRepository {
            db_connection: db_connection.clone(),
        },
    });

    let port = listner.local_addr().unwrap().port();

    let redis_url = dotenvy::var("REDIS_URL").expect("REDIS_URL must be set");
    let secret_key = Key::generate();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(SessionMiddleware::new(RedisActorSessionStore::new(redis_url.clone()), secret_key.clone()))
            // enable logger - always register Actix web Logger middleware last
            .wrap(Logger::default())
            .configure(adapters::api::shared::routes::routes)
    })
    .listen(listner)?
    .run();

    println!("Server running on port {}, db_name {}", port, db_name);

    Ok(server)
}
