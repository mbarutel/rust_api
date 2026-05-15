use chrono::{DateTime, NaiveDateTime, Utc};

use crate::{
    application::entity::VenueEntity,
    domain::models::{GroupDiscount, conference::Conference, venue::Venue},
};

#[derive(Debug, sqlx::FromRow)]
pub struct ConferenceEntity {
    pub id: u64,
    pub code: String,
    pub name: String,
    pub poster_url: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub venue_id: Option<u64>,
    pub group_discount_id: Option<u64>,
    pub published: i8,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ConferenceEntity {
    pub fn is_published(&self) -> bool {
        self.published != 0
    }
}

// impl From<(ConferenceEntity, Option<VenueEntity>)> for Conference {
//     fn from((conference_entity, venue_entity): (ConferenceEntity, Option<VenueEntity>)) -> Self {
//         let published = conference_entity.is_published();
//         let venue = venue_entity.map(Venue::from);

//         Conference {
//             id: conference_entity.id,
//             code: conference_entity.code,
//             name: conference_entity.name,
//             poster_url: conference_entity.poster_url,
//             description: conference_entity.description,
//             start_date: conference_entity.start_date,
//             end_date: conference_entity.end_date,
//             venue,
//             published,
//             created_at: conference_entity.created_at,
//             updated_at: conference_entity.updated_at,
//         }
//     }
// }

impl From<ConferenceEntity> for Conference {
    fn from(conference_entity: ConferenceEntity) -> Self {
        let is_published = conference_entity.is_published();

        Self {
            id: conference_entity.id,
            code: conference_entity.code,
            name: conference_entity.name,
            poster_url: conference_entity.poster_url,
            description: conference_entity.description,
            start_date: conference_entity.start_date,
            end_date: conference_entity.end_date,
            venue: None,
            group_discount: None,
            is_published,
            created_at: conference_entity.created_at,
            updated_at: conference_entity.updated_at,
        }
    }
}

impl Conference {
    pub fn with_venue(mut self, venue: Option<VenueEntity>) -> Self {
        self.venue = venue.map(Venue::from);
        self
    }

    pub fn with_group_discount(mut self, group_discount: Option<GroupDiscount>) -> Self {
        self.group_discount = group_discount.map(GroupDiscount::from);
        self
    }
}
