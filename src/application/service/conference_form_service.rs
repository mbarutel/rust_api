use crate::application::{dto::registration_dto::RegistrationFormData, error::AppError};

pub trait ConferenceFormService: Send + Sync {
    async fn get_registration_form_data(
        &self,
        conference_id: u64,
    ) -> Result<RegistrationFormData, AppError>;
}
