use crate::error::AppError;
use actix_session::Session;
use uuid::Uuid;

pub fn validate_session(session: &Session) -> Result<(), AppError> {
    if session.get::<Uuid>("session_id")?.is_none() {
        return Err(AppError::Unauthorized("Inavlid session".into()));
    } else {
        Ok(())
    }
}

pub fn generate_session_id() -> String {
    Uuid::new_v4().to_string()
}

pub fn set_session_id(session: &Session, session_id: &str) -> Result<(), AppError> {
    session.insert("session_id", "hi")?;
    Ok(())
}
