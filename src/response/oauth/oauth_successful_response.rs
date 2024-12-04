use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct OauthSuccessfulResponse {
    /// The access token string as issued by the authorization server
    pub access_token: String,

    /// The type of token this is, typically just the string `Bearer`
    pub token_type: String,

    /// If the access token expires, the server should reply with the duration
    /// of time the access token is granted for
    pub expires_in: u32,

    #[serde(default)]
    /// If the access token will expire, then it is useful to return a refresh
    /// token which applications can use to obtain another access token. However,
    /// tokens issued with the implicit grant cannot be issued a refresh token
    pub refresh_token: String,

    #[serde(default)]
    /// If the scope the user granted is identical to the scope the app requested,
    /// this parameter is optional. If the granted scope is different from the requested
    /// scope, such as if the user modified the scope, then this parameter is required
    pub scope: String,
}
