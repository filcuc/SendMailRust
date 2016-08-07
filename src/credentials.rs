use oauth2_flow::*;
use rustc_serialize::json;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::Read;
use std::io;

#[derive(Debug)]
pub enum CredentialsError {
    GenericError,
    IOError(io::Error),
    EncoderError(json::EncoderError),
    DecoderError(json::DecoderError),
    TokenExchangeError,
    AccessTokenRefreshError,
    OAuth2Error(OAuth2FlowError)
}

impl From<json::EncoderError> for CredentialsError {
    fn from(err: json::EncoderError) -> CredentialsError { CredentialsError::EncoderError(err) }
}

impl From<json::DecoderError> for CredentialsError {
    fn from(err: json::DecoderError) -> CredentialsError { CredentialsError::DecoderError(err) }
}

impl From<io::Error> for CredentialsError {
    fn from(err: io::Error) -> CredentialsError  { CredentialsError::IOError(err) }
}

impl From<OAuth2FlowError> for CredentialsError {
    fn from(err: OAuth2FlowError) -> CredentialsError { CredentialsError::OAuth2Error(err) }
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Credentials {
    pub access_token_data: AccessTokenData,
    pub client_id: String,
    pub client_secret: String,
}

impl Credentials {
    /// Setup the Credentials through an oauth authentication flow
    pub fn setup(client_id: String, client_secret: String) -> Result<Credentials, CredentialsError> {
        let flow = Credentials::create_oauth_flow(client_id.clone(), client_secret.clone());
        let authorize_url = flow.step_1_get_authorize_url();
        println!("Please open a browser at the following url {} and grant access to this application", authorize_url);
        println!("Paste here the obtained access code");
        let mut access_code = String::new();
        try!(io::stdin().read_line(&mut access_code));
        println!("Exchanging {} for an access_token", access_code);
        let access_token_data = try!(flow.step_2_exchange_access_code(access_code.as_str()));
        println!("Obtained {} token", access_token_data.access_token);
        let result = Credentials {
            access_token_data: access_token_data,
            client_id: client_id,
            client_secret: client_secret
        };
        try!(result.save());
        Ok(result)
    }

    /// Load a Credentials stuct from the json file
    fn load() -> Result<Credentials, CredentialsError> {
        println!("Restoring credentials from json file");
        let mut file = try!(File::open("credentials.json"));
        let mut content = String::new();
        try!(file.read_to_string(&mut content));
        let result = try!(json::decode::<Credentials>(content.as_str()));
        Ok(result)
    }

    /// Refresh the Credentials file
    fn refresh(&mut self) -> Result<(), CredentialsError> {
        println!("Refreshing the credentials");
        let flow = Credentials::create_oauth_flow(self.client_id.clone(), self.client_secret.clone());
        self.access_token_data = try!(flow.refresh_access_token(&self.access_token_data));
        Ok(())
    }

    /// Load the Credentials from the config file and refresh them
    pub fn load_and_refresh() -> Result<Credentials, CredentialsError> {
        let mut credentials = try!(Credentials::load());
        try!(credentials.refresh());
        Ok(credentials)
    }

    /// Save the Credentials to the json file
    fn save(&self) -> Result<(), CredentialsError> {
        println!("Saving the credentials");
        let mut f = try!(OpenOptions::new().create(true).write(true).open("credentials.json"));
        let encoded = try!(json::encode(self));
        try!(f.write_all(encoded.as_bytes()));
        try!(f.sync_all());
        Ok(())
    }

    /// Create an oauth authentication flow
    fn create_oauth_flow(client_id: String, client_secret: String) -> OAuth2Flow {
        OAuth2Flow {
            client_id: client_id,
            client_secret: client_secret,
            scopes: vec!("https://www.googleapis.com/auth/gmail.send".to_string()),
            redirect_uri: "urn:ietf:wg:oauth:2.0:oob".to_string(),
            auth_uri: "https://accounts.google.com/o/oauth2/auth".to_string(),
            token_uri: "https://accounts.google.com/o/oauth2/token".to_string()
        }
    }
}
