use async_trait::async_trait;

use crate::{
    application::{repositories::dog_facts_repository_abstract::DogFactsRepositoryAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{dog_fact_entity::DogFactEntity, error::ApiError},
};

pub struct GetAllDogFactsUseCase<'a> {
    repository: &'a dyn DogFactsRepositoryAbstract,
}

impl<'a> GetAllDogFactsUseCase<'a> {
    pub fn new(repository: &'a dyn DogFactsRepositoryAbstract) -> Self {
        GetAllDogFactsUseCase { repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<Vec<DogFactEntity>> for GetAllDogFactsUseCase<'a> {
    async fn execute(&self) -> Result<Vec<DogFactEntity>, ApiError> {
        let dog_facts = self.repository.get_all_dog_facts().await;

        match dog_facts {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get all dog facts", Some(e))),
        }
    }
}
