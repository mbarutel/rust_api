use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, PartialEq)]
pub enum RegistrationStatus {
    Submitted,
    Accepted,
    Waitlisted,
    Rejected,
    Withdrawn,
}

impl RegistrationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Submitted => "submitted",
            Self::Accepted => "accepted",
            Self::Waitlisted => "waitlisted",
            Self::Rejected => "rejected",
            Self::Withdrawn => "withdrawn",
        }
    }

    pub fn can_transition_to(&self, next: &RegistrationStatus) -> bool {
        matches!(
            (self, next),
            (Self::Submitted, Self::Accepted)
                | (Self::Submitted, Self::Waitlisted)
                | (Self::Submitted, Self::Rejected)
                | (Self::Submitted, Self::Withdrawn)
                | (Self::Waitlisted, Self::Accepted)
                | (Self::Waitlisted, Self::Withdrawn)
                | (Self::Accepted, Self::Withdrawn)
        )
    }
}

impl TryFrom<&str> for RegistrationStatus {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "submitted" => Ok(Self::Submitted),
            "accepted" => Ok(Self::Accepted),
            "waitlisted" => Ok(Self::Waitlisted),
            "rejected" => Ok(Self::Rejected),
            "withdrawn" => Ok(Self::Withdrawn),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PaymentStatus {
    Unpaid,
    Partial,
    Paid,
}

#[derive(Debug, Clone)]
pub struct Registration {
    pub id: u64,
    pub conference_id: u64,
    pub status: RegistrationStatus,
    pub cost: Decimal,
    pub discount_code: Option<String>,
    pub discount_amount: Decimal,
    pub amount_paid: Decimal,
    pub created_by_id: Option<u64>,
    pub notes_internal: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Registration {
    pub fn payment_status(&self) -> PaymentStatus {
        if self.amount_paid.is_zero() {
            PaymentStatus::Unpaid
        } else if self.amount_paid >= self.cost {
            PaymentStatus::Paid
        } else {
            PaymentStatus::Partial
        }
    }
}
