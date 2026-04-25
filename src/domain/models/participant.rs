use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub enum ParticipantRole {
    Delegate,
    Speaker,
    Sponsor,
    Exhibitor,
}

impl ParticipantRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Delegate => "delegate",
            Self::Speaker => "speaker",
            Self::Sponsor => "sponsor",
            Self::Exhibitor => "exhibitor",
        }
    }
}

impl TryFrom<&str> for ParticipantRole {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "delegate" => Ok(Self::Delegate),
            "speaker" => Ok(Self::Speaker),
            "sponsor" => Ok(Self::Sponsor),
            "exhibitor" => Ok(Self::Exhibitor),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Participant {
    pub id: u64,
    pub registration_id: u64,
    pub client_id: u64,
    pub role: ParticipantRole,
    pub dietary_requirements: Option<String>,
    pub accessibility_needs: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
