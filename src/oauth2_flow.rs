
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
       "".to_string()
    }

    pub fn step_2_exchange_access_code(&self, access_code: &str) -> String {
        "".to_string()
    }
}
