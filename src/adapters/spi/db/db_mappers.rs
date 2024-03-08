use crate::adapters::spi::db::models::DogFact;
use crate::adapters::spi::db::models::Invitation;
use crate::adapters::spi::db::models::User;
use crate::application::mappers::db_mapper::DbMapper;
use crate::domain::dog_fact_entity::DogFactEntity;
use crate::domain::invitation_entity::InvitationEntity;
use crate::domain::user_entity::UserEntity;
use uuid::Uuid;

pub struct DogFactDbMapper {}

impl DbMapper<DogFactEntity, DogFact> for DogFactDbMapper {
    fn to_db(entity: DogFactEntity) -> DogFact {
        DogFact {
            id: entity.fact_id,
            fact: entity.fact,
        }
    }

    fn to_entity(model: DogFact) -> DogFactEntity {
        DogFactEntity {
            fact_id: model.id,
            fact: model.fact,
        }
    }
}

pub struct InvitationDbMapper {}

impl DbMapper<InvitationEntity, Invitation> for InvitationDbMapper {
    fn to_db(entity: InvitationEntity) -> Invitation {
        Invitation {
            invitation_token: entity.invitation_token,
            email: entity.email,
            used: entity.used,
            expires_at: entity.expires_at,
        }
    }

    fn to_entity(model: Invitation) -> InvitationEntity {
        InvitationEntity {
            invitation_token: model.invitation_token,
            email: model.email,
            used: model.used,
            expires_at: model.expires_at,
        }
    }
}

pub struct RegisterCompleteDbMapper {}

impl DbMapper<UserEntity, User> for RegisterCompleteDbMapper {
    fn to_db(entity: UserEntity) -> User {
        User {
            id: Uuid::new_v4(),
            email: entity.email,
            password: entity.password,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }

    fn to_entity(model: User) -> UserEntity {
        UserEntity {
            email: model.email,
            password: model.password,
        }
    }
}

pub struct LoginDbMapper {}

impl DbMapper<UserEntity, User> for LoginDbMapper {
    fn to_db(entity: UserEntity) -> User {
        User {
            id: Uuid::new_v4(),
            email: entity.email,
            password: entity.password,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }

    fn to_entity(model: User) -> UserEntity {
        UserEntity {
            email: model.email,
            password: model.password,
        }
    }
}
