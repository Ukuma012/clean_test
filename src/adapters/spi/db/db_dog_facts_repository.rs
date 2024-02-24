use async_trait::async_trait;
use diesel::prelude::*;
use std::error::Error;

use crate::adapters::spi::db::{db_connection::DbConnection, models::DogFact, schema::dog_facts::dsl::*};

pub struct DogFactsRepository {
    pub db_connection: DbConnection,
}

// #[async_trait(?Send)]
// impl DogFactsRepositoryAbstract for DogFactsRepository {}
