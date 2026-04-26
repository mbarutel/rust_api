use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::models::registration::{PaymentStatus, Registration};

// const INITIAL_SUBMISSION: SubmissionType = {
//   conferenceTitle: undefined,
//   selectedConference: undefined,
//   selectedPriceTier: undefined,
//   delegates: [
//     {
//       firstName: "",
//       lastName: "",
//       jobTitle: "",
//       organization: "",
//       email: "",
//       phone: "",
//       diet: "normal",
//       dinner: false,
//       masterclass: null,
//       accommodationNights: 0,
//     },
//   ],
//   promoCode: "",
//   reference: "Manager, Family, Friend or Colleague",
// };

// #[derive(Debug, Deserialize, Validate)]
// pub struct RegisterDelegateRequest {
//     pub conference_id: u64,
//     pub cost: Option<Decimal>,
//     pub discount_code: Option<String>,
//     pub discount_amount: Option<Decimal>,
//     pub main_participant:
// }

// #[derive(Debug, Deserialize, Validate)]
// pub struct RegisterSpeakerRequest {
//     pub conference_id: u64,
// }

// #[derive(Debug, Deserialize, Validate)]
// pub struct RegisterExhibitorRequest {
//     pub conference_id: u64,
// }

// #[derive(Debug, Deserialize, Validate)]
// pub struct RegisterSponsorRequest {
//     pub conference_id: u64,
// }

// #[derive(Debug, Deserialize, Validate)]
// pub struct SubmitDelegateRequest {
//     pub conference_id: u64,
// }
#[derive(Debug, Deserialize, Validate)]
struct ParticipantInfo {
    first_name: String,
    last_name: String,
    job_title: String,
    organization_name: String,
    #[validate(email)]
    email: String,
    dietary_requirements: String,
    networking_dinner: bool,
    masterclass_selection: Option<u64>,
    accomodation_nights: u8,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterDelegateRequest {
    pub conference_id: u64,
    pub cost: Decimal,
    pub discount_code: Option<String>,
    pub delegates: Vec<ParticipantInfo>,
}

// pub struct RegisterDelegateFormRequest

#[derive(Debug, Deserialize, Validate)]
pub struct CreateRegistrationRequest {
    pub conference_id: u64,
    pub created_by_id: Option<u64>,
    pub cost: Option<Decimal>,
    pub discount_code: Option<String>,
    pub discount_amount: Option<Decimal>,
    pub notes_internal: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateRegistrationRequest {
    pub cost: Option<Decimal>,
    pub discount_code: Option<String>,
    pub discount_amount: Option<Decimal>,
    pub notes_internal: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct TransitionStatusRequest {
    #[validate(length(min = 1))]
    pub status: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RecordPaymentRequest {
    pub amount: Decimal,
}

#[derive(Debug, Serialize)]
pub struct RegistrationResponse {
    pub id: u64,
    pub conference_id: u64,
    pub status: String,
    pub payment_status: String,
    pub cost: Decimal,
    pub discount_code: Option<String>,
    pub discount_amount: Decimal,
    pub amount_paid: Decimal,
    pub created_by_id: Option<u64>,
    pub notes_internal: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Registration> for RegistrationResponse {
    fn from(r: Registration) -> Self {
        let payment_status = match r.payment_status() {
            PaymentStatus::Unpaid => "unpaid",
            PaymentStatus::Partial => "partial",
            PaymentStatus::Paid => "paid",
        };
        RegistrationResponse {
            id: r.id,
            conference_id: r.conference_id,
            status: r.status.as_str().to_string(),
            payment_status: payment_status.to_string(),
            cost: r.cost,
            discount_code: r.discount_code,
            discount_amount: r.discount_amount,
            amount_paid: r.amount_paid,
            created_by_id: r.created_by_id,
            notes_internal: r.notes_internal,
            created_at: r.created_at.to_string(),
            updated_at: r.updated_at.to_string(),
        }
    }
}
