use crate::domain::error::ApiError;
use crate::error::AppError;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait AbstractUseCase<T> {
    async fn execute(&self) -> Result<T, ApiError>;
}

#[async_trait(?Send)]
pub trait AbstractInvitationUseCase<T> {
    async fn post_invitation(&self) -> Result<T, ApiError>;
}

#[async_trait(?Send)]
pub trait AbstractRegisterCompleteUseCase<T> {
    async fn register(&self) -> Result<T, AppError>;
}
