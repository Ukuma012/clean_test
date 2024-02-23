use dotenvy;
use std::env;
use std::net::TcpListener;

use test_api::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let environment = match env::var("APP_ENVIRONMENT") {
        Ok(val) => val,
        Err(_) => "development".to_string(),
    };

    dotenvy::from_filename(".env.".to_string() + &environment).ok();

    let listener = TcpListener::bind("0.0.0.0:8888").expect("Failed to bind random port");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

    run(listener, &database_name)?.await
}
