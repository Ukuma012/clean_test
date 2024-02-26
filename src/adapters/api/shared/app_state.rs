use crate::adapters::spi::db::db_dog_facts_repository::DogFactsRepository;
use crate::adapters::spi::db::db_invitation_repository::InvitationRepository;

pub struct AppState {
    pub app_name: String,
    pub dogs_repository: DogFactsRepository,
    pub invitation_repository: InvitationRepository,
}
