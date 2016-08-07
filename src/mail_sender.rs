use credentials::{Credentials};
use std::io::Read;
use hyper::{Client, Url};
use hyper::header::{Headers, Authorization, Bearer, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};
use url::percent_encoding::{DEFAULT_ENCODE_SET, utf8_percent_encode};
use rustc_serialize::json;
use std::str::FromStr;


enum SendMailError {
    UrlParsingError,
    NetworkError
}

pub struct MailSender<'a> {
    credentials: &'a Credentials
}

impl<'a> MailSender<'a> {
    pub fn new(credentials: &'a Credentials) -> MailSender {
        MailSender { credentials: credentials }
    }

    pub fn send(&self, from: &str, to: &str, subject: &str, body: &str) -> Result<(),SendMailError> {
        println!("Sending an email from {} to {} with subject {}", from , to, subject);
        let url = try!(Url::parse("https://www.googleapis.com/upload/gmail/v1/users/me/messages/send").map_err(|_|SendMailError::UrlParsingError));
        let mut headers = Headers::new();
        headers.set(Authorization(Bearer{token: self.credentials.access_token_data.access_token.clone()}));
        headers.set(ContentType(Mime::from_str("message/rfc822").unwrap()));
        let body = format!("From:{}\nTo:{}\nSubject:{}\n\n{}\n", from, to, subject, body);
        let client = Client::new();
        let mut res = try!{client.post(url).headers(headers).body(body.as_str()).send().map_err(|_|SendMailError::NetworkError)};
        Ok(())
    }
}
