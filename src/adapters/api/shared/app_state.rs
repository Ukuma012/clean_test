use crate::adapters::spi::db::db_dog_facts_repository::DogFactsRepository;
use crate::adapters::spi::db::db_invitation_repository::InvitationRepository;
use crate::adapters::spi::db::db_register_complete_repository::RegisterCompleteRepository;
use crate::adapters::spi::email::email_repository::MailTrapRepository;

pub struct AppState {
    pub app_name: String,
    pub dogs_repository: DogFactsRepository,
    pub invitation_repository: InvitationRepository,
    pub email_repository: MailTrapRepository,
    pub register_complete_repository: RegisterCompleteRepository,
}
