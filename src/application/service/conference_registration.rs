use crate::application::{
    dto::registration::{RegisterDelegateRequest, RegistrationResponse},
    error::AppError,
};

#[cfg_attr(test, mockall::automock)]
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
