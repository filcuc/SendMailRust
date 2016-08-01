use std::io::Read;
use hyper::{Client, Url};
use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};
use url::percent_encoding::{DEFAULT_ENCODE_SET, utf8_percent_encode};
use rustc_serialize::json;

pub enum OAuth2FlowError {
    InvalidUrlError,
    NetworkError,
    AccessTokenParsingError
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct AccessTokenData {
    access_token: String,
    token_type: String,
    expires_in: i64,
    refresh_token: String
}

pub struct OAuth2Flow {
    pub client_id: String,
    pub client_secret: String,
    pub scopes: Vec<String>,
    pub redirect_uri: String,
    pub auth_uri: String,
    pub token_uri: String
}

impl OAuth2Flow {
    pub fn step_1_get_authorize_url(&self) -> String {
        format!("{}?response_type=code&client_id={}&redirect_uri={}&scope={}",
                self.auth_uri, self.client_id, self.redirect_uri,
                self.percent_encode_scopes())
    }

    pub fn step_2_exchange_access_code(&self, access_code: &str) -> Result<AccessTokenData, OAuth2FlowError> {
        let client = Client::new();
        let url = try!{Url::parse(self.token_uri.as_str()).map_err(|_|OAuth2FlowError::InvalidUrlError)};
        let mut headers = Headers::new();
        headers.set(ContentType(Mime(TopLevel::Application, SubLevel::WwwFormUrlEncoded, vec![])));
        let body = format!("code={}&client_id={}&client_secret={}&redirect_uri={}&grant_type={}",
                           access_code, self.client_id, self.client_secret,
                           self.redirect_uri, "authorization_code");
        let mut res = try!{client.post(url).headers(headers).body(body.as_str()).send().map_err(|_|OAuth2FlowError::NetworkError)};
        let mut res_body = String::new();
        try!{res.read_to_string(&mut res_body).map_err(|_|OAuth2FlowError::NetworkError)};
        println!("{}", res_body);
        let result = try!{json::decode::<AccessTokenData>(res_body.as_str()).map_err(|_|OAuth2FlowError::AccessTokenParsingError)};
        Ok(result)
    }

    pub fn refresh_access_token(&self) {
    }

    fn percent_encode_scopes(&self) -> String {
        let scopes = self.scopes.join(" ");
        let result = utf8_percent_encode(scopes.as_str(), DEFAULT_ENCODE_SET);
        format!("{}", result)
    }
}
