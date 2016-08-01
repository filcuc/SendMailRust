use oauth2_flow::*;
use rustc_serialize::json;
use rustc_serialize::Decodable;
use rustc_serialize::Encodable;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::Read;
use std::io;
use std::vec;

#[derive(Debug)]
pub enum CredentialsError {
    GenericError,
    IOError(io::Error),
    EncoderError(json::EncoderError),
    DecoderError(json::DecoderError),
    TokenExchangeError,
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

#[derive(RustcDecodable, RustcEncodable)]
pub struct Credentials {
    access_token: String,
    refresh_token: String,
    expires_in: i64,
    client_id: String,
    client_secret: String,
}

impl Credentials {
    pub fn setup(client_id: String, client_secret: String) -> Result<Credentials, CredentialsError> {
        let flow = OAuth2Flow {
            client_id: client_id.clone(),
            client_secret: client_secret.clone(),
            scopes: vec!("https://www.googleapis.com/auth/gmail.send".to_string()),
            redirect_uri: "urn:ietf:wg:oauth:2.0:oob".to_string(),
            auth_uri: "https://accounts.google.com/o/oauth2/auth".to_string(),
            token_uri: "https://accounts.google.com/o/oauth2/token".to_string()
        };
        let authorize_url = flow.step_1_get_authorize_url();
        println!("Please open a browser at the following url {} and grant access to this application", authorize_url);
        println!("Paste here the obtained access code");
        let mut access_code = String::new();
        try!{ io::stdin().read_line(&mut access_code)};
        println!("Exchanging {} for an access_token", access_code);
        let access_token = try!{flow.step_2_exchange_access_code(access_code.as_str()).map_err(|_|CredentialsError::TokenExchangeError)};
        println!("Obtained {} token", access_token);
        let result = Credentials {
            access_token: access_token,
            refresh_token: "".to_string(),
            expires_in: 3600,
            client_id: client_id,
            client_secret: client_secret
        };
        try!{result.save()};
        Ok(result)
    }

    pub fn load() -> Result<Credentials, CredentialsError> {
        let mut file = try!(File::open("credentials.json"));
        let mut content = String::new();
        try!(file.read_to_string(&mut content));
        let result = try!(json::decode::<Credentials>(content.as_str()));
        Ok(result)
    }

    fn save(&self) -> Result<(), CredentialsError> {
        let mut f = try!(OpenOptions::new().create(true).write(true).open("credentials.json"));
        let encoded = try!(json::encode(self));
        try!(f.write_all(encoded.as_bytes()));
        try!(f.sync_all());
        Ok(())
    }
}
