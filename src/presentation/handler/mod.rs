pub mod activity;
pub mod auth;
pub mod client;
pub mod conference;
pub mod exhibitor;
pub mod health;
pub mod masterclass;
pub mod organization;
pub mod participant;
pub mod registration;
pub mod speaker;
pub mod sponsor;
pub mod user;
pub mod venue;

pub use activity::activity_routes;
pub use auth::auth_routes;
pub use client::client_routes;
pub use conference::conference_routes;
pub use exhibitor::exhibitor_routes;
pub use health::health_routes;
pub use masterclass::masterclass_routes;
pub use organization::organization_routes;
pub use participant::participant_routes;
pub use registration::registration_routes;
pub use speaker::speaker_routes;
pub use sponsor::sponsor_routes;
pub use user::user_routes;
pub use venue::venue_routes;

#[cfg(test)]
mod utils;
