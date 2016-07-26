use url::percent_encoding::DEFAULT_ENCODE_SET;
use url::percent_encoding::utf8_percent_encode;

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
        let scope = {
            let mut result = "".to_string();
            for i in 0..self.scopes.len() {
                if i != 0 {
                    result = result + " ";
                }
                let as_percent = utf8_percent_encode(self.scopes[i].as_str(), DEFAULT_ENCODE_SET);
                let as_str = format!("{}", as_percent);
                result = result + &as_str;
            }
            result
        };

        println!("{}", scope);

        format!("{}?response_type=code&client_id={}&redirect_uri={}&scope={}",
                self.auth_uri, self.client_id, self.redirect_uri, scope)

    }

    pub fn step_2_exchange_access_code(&self, access_code: &str) -> String {
        "".to_string()
    }
}
