use credentials::Credentials;
use hyper::{Client, Url};
use hyper::header::{Headers, Authorization, Bearer, ContentType};
use hyper::mime::Mime;
use hyper::status::StatusCode;
use std::str::FromStr;

pub enum SendMailError {
    UrlParsingError,
    NetworkError,
}

pub struct MailSender<'a> {
    credentials: &'a Credentials,
}

impl<'a> MailSender<'a> {
    pub fn new(credentials: &'a Credentials) -> MailSender {
        MailSender { credentials: credentials }
    }

    pub fn send(&self,
                from: &str,
                to: &str,
                subject: &str,
                body: &str)
                -> Result<(), SendMailError> {
        println!("Sending an email from {} to {} with subject {}",
                 from,
                 to,
                 subject);
        let url = try!(Url::parse("https://www.googleapis.\
                                   com/upload/gmail/v1/users/me/messages/send")
            .map_err(|_| SendMailError::UrlParsingError));
        let mut headers = Headers::new();
        headers.set(Authorization(Bearer {
            token: self.credentials.access_token_data.access_token.clone(),
        }));
        headers.set(ContentType(Mime::from_str("message/rfc822").unwrap()));
        let body = format!("From:{}\nTo:{}\nSubject:{}\n\n{}\n",
                           from,
                           to,
                           subject,
                           body);
        let client = Client::new();
        let res = try!(client.post(url)
            .headers(headers)
            .body(body.as_str())
            .send()
            .map_err(|_| SendMailError::NetworkError));
        match res.status {
            StatusCode::Ok => Ok(()),
            _ => Err(SendMailError::NetworkError),
        }
    }
}
