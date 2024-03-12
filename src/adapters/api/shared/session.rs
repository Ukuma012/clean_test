use crate::error::AppError;
use actix_session::Session;
use uuid::Uuid;

pub fn validate_session(session: Session) -> Result<(), AppError> {
    let text: Option<String> = session.get("testing")?;
    println!("{:?}", text);

    let user_text: Option<String> = session.get("user_id")?;
    println!("{:?}", user_text);
    Ok(())
}

pub fn generate_session_id() -> String {
    Uuid::new_v4().to_string()
}

pub fn set_session_id(session: &Session, session_id: &str) -> Result<(), AppError> {
    session.insert("session_id", "hi")?;
    session.insert("testing", "can you see me?")?;
    Ok(())
}
