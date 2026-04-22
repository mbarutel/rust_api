use crate::application::{entity::conference_entity::ConferenceEntity, repository::Repository};

pub trait ConferenceRepository: Repository<ConferenceEntity> {}
