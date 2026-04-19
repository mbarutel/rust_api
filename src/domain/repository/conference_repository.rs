use crate::domain::models::conference::Conference;
use crate::domain::repository::Repository;

pub trait ConferenceRepository: Repository<Conference> {}
