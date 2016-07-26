extern crate rustc_serialize;
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
    pub fn setup(client_id: &str, client_secret: &str) -> Result<Credentials, CredentialsError> {
        Err(CredentialsError::GenericError)
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