use axum::{
    Json,
    extract::{FromRequest, Request},
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::error::AppError;

// JSON extractor that automatically validates the payload
pub struct ValidateJson<T>(pub T);

impl<St, T> FromRequest<St> for ValidateJson<T>
where
    T: DeserializeOwned + Validate,
    St: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &St) -> Result<Self, Self::Rejection> {
        // Extract JSON body
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e| AppError::Validation(e.to_string()))?;

        // Validate
        value.validate().map_err(|e| {
            let errors: Vec<String> = e
                .field_errors()
                .into_iter()
                .flat_map(|(field, errors)| {
                    errors.iter().map(move |e| {
                        format!(
                            "{}: {}",
                            field,
                            e.message
                                .as_ref()
                                .map(|m| m.to_string())
                                .unwrap_or_default()
                        )
                    })
                })
                .collect();
            AppError::Validation(errors.join(","))
        })?;

        Ok(ValidateJson(value))
    }
}
