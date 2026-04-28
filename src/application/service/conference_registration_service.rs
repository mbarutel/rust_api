use crate::application::{
    dto::registration_dto::{RegisterDelegateRequest, RegistrationResponse},
    error::AppError,
};

#[async_trait::async_trait]
pub trait ConferenceRegistrationService: Send + Sync {
    async fn register_delegates(
        &self,
        dto: RegisterDelegateRequest,
    ) -> Result<RegistrationResponse, AppError>;

    // async fn register_speakers(
    //     &self,
    //     dto: RegisterDelegateRequest,
    // ) -> Result<RegistrationResponse, AppError>;

    // async fn register_exhibitor(
    //     &self,
    //     dto: RegisterDelegateRequest,
    // ) -> Result<RegistrationResponse, AppError>;

    // async fn register_sponsor(
    //     &self,
    //     dto: RegisterDelegateRequest,
    // ) -> Result<RegistrationResponse, AppError>;
}
