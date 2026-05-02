use crate::application::{dto::registration_dto::RegistrationFormData, error::AppError};

#[async_trait::async_trait]
pub trait ConferenceFormService: Send + Sync {
    async fn get_registration_form_data(
        &self,
        conference_id: u64,
    ) -> Result<RegistrationFormData, AppError>;
}
