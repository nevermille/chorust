use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct OauthErrorResponse {
    pub error: String,

    #[serde(default)]
    pub error_description: String,

    #[serde(default)]
    pub error_uri: String,
}

impl OauthErrorResponse {
    /// Returns the error value in the enums form
    pub fn error_to_enum(&self) -> OauthError {
        (&self.error).into()
    }
}

pub enum OauthError {
    InvalidRequest,
    InvalidClient,
    InvalidGrant,
    InvalidScope,
    UnauthorizedClient,
    UnsupportedGrantType,
    Unknown(String),
}

impl From<&String> for OauthError {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "invalid_request" => OauthError::InvalidRequest,
            "invalid_client" => OauthError::InvalidClient,
            "invalid_grant" => OauthError::InvalidGrant,
            "invalid_scope" => OauthError::InvalidScope,
            "unauthorized_client" => OauthError::UnauthorizedClient,
            "unsupported_grant_type" => OauthError::UnsupportedGrantType,
            v => OauthError::Unknown(v.to_string()),
        }
    }
}

impl From<String> for OauthError {
    fn from(value: String) -> Self {
        (&value).into()
    }
}
