use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CertificateStatus {
    Active,
    Revoked,
    Expired,
}

impl CertificateStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "ACTIVE",
            Self::Revoked => "REVOKED",
            Self::Expired => "EXPIRED",
        }
    }
}

impl TryFrom<String> for CertificateStatus {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "ACTIVE" => Ok(Self::Active),
            "REVOKED" => Ok(Self::Revoked),
            "EXPIRED" => Ok(Self::Expired),
            _ => Err(format!("unsupported certificate status: {value}")),
        }
    }
}
