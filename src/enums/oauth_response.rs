use crate::response::oauth::{OauthErrorResponse, OauthSuccessfulResponse};

pub enum OAuthResponse {
    Success(OauthSuccessfulResponse),
    Error(OauthErrorResponse),
    Unknown(String),
}

impl OAuthResponse {
    pub fn from_json(json: &str) -> anyhow::Result<OAuthResponse> {
        let try_success = serde_json::from_str::<OauthSuccessfulResponse>(json);
        let try_error = serde_json::from_str::<OauthErrorResponse>(json);

        match (try_success, try_error) {
            (Ok(v), _) => Ok(OAuthResponse::Success(v)),
            (_, Ok(v)) => Ok(OAuthResponse::Error(v)),
            (_, _) => Ok(OAuthResponse::Unknown(json.to_string())),
        }
    }
}
