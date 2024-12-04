mod factures;

use crate::enums::OAuthResponse;
use base64::prelude::*;
use base64::Engine;
use curl::easy::{Easy, List};
use form_urlencoded::Serializer;
use squared_api_wrapper::response::StringObjectResponse;
use squared_api_wrapper::traits::Api;
use url_builder::URLBuilder;

#[derive(Default)]
pub struct Chorus {
    pub oauth_client_id: String,

    pub oauth_client_secret: String,

    pub cpro_account: String,

    pub bearer_token: String,

    pub root_url: String,

    pub oauth_url: String,
}

impl Chorus {
    pub fn switch_to_sandbox_mode(&mut self, sandbox_mode: bool) {
        match sandbox_mode {
            true => {
                self.root_url = "sandbox-api.piste.gouv.fr".to_string();
                self.oauth_url = "sandbox-oauth.piste.gouv.fr".to_string();
            }
            false => {
                self.root_url = "api.piste.gouv.fr".to_string();
                self.oauth_url = "oauth.piste.gouv.fr".to_string();
            }
        }
    }

    fn oauth_connect_url(&self) -> String {
        let mut url = URLBuilder::new();
        url.set_protocol("https")
            .set_host(&self.oauth_url)
            .add_route("api")
            .add_route("oauth")
            .add_route("token");

        url.build()
    }

    pub fn connect_with_oauth(
        &mut self,
        client_id: &str,
        client_secret: &str,
    ) -> anyhow::Result<StringObjectResponse<OAuthResponse>> {
        // We will need special headers for this URL
        let mut curl = self.get_easy_base()?;

        // We will use Oauth URL
        curl.url(&self.oauth_connect_url())?;

        // Set OAuth headers
        let mut headers = List::new();

        squared_api_wrapper::add_header(
            &mut headers,
            "Content-type",
            "application/x-www-form-urlencoded",
        )?;
        curl.http_headers(headers)?;

        // Set OAuth form
        let mut form = Serializer::new(String::new());
        form.append_pair("grant_type", "client_credentials");
        form.append_pair("client_id", client_id);
        form.append_pair("client_secret", client_secret);
        form.append_pair("scope", "openid");

        let form = form.finish();

        let connect =
            squared_api_wrapper::post(&mut curl, Some(form.as_bytes()), None)?.to_string_response();
        let object = OAuthResponse::from_json(&connect.raw_data)?;

        // We save the information if connection was succesful
        if let OAuthResponse::Success(v) = &object {
            self.oauth_client_id = client_id.to_string();
            self.oauth_client_secret = client_secret.to_string();
            self.bearer_token = v.access_token.clone();
        }

        let connect = connect.add_object(object);

        Ok(connect)
    }

    pub fn connect_with_bearer_token(&mut self, bearer_token: &str) {
        self.bearer_token = bearer_token.to_string();
    }

    pub fn set_choruspro_account(&mut self, username: &str, password: &str) {
        let to_encode = format!("{username}:{password}");
        self.cpro_account = BASE64_STANDARD.encode(to_encode.as_bytes());
    }
}

impl Api for Chorus {
    fn get_easy_base(&self) -> anyhow::Result<Easy> {
        let mut curl = Easy::new();

        curl.follow_location(true)?;

        Ok(curl)
    }

    fn get_headers_base(&self) -> anyhow::Result<List> {
        let mut headers = List::new();
        let bearer_header = format!("Bearer {}", &self.bearer_token);

        squared_api_wrapper::add_header(&mut headers, "cpro-account", &self.cpro_account)?;
        squared_api_wrapper::add_header(&mut headers, "Authorization", &bearer_header)?;
        squared_api_wrapper::add_header(
            &mut headers,
            "Content-Type",
            "application/json;charset=utf-8",
        )?;

        Ok(headers)
    }

    fn get_root_url(&self) -> String {
        self.root_url.clone()
    }
}
