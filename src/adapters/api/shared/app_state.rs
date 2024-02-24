use crate::adapters::spi::db::db_dog_facts_repository::DogFactsRepository;

pub struct AppState {
    pub app_name: String,
    pub dogs_repository: DogFactsRepository,
}
