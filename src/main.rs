mod oauth2_flow;
extern crate getopts;
extern crate rustc_serialize;
use rustc_serialize::json;
use getopts::Options;
use std::env;
use std::vec;
use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
use oauth2_flow::OAuth2Flow;

enum SendMailError {
    Parse
}

#[derive(RustcDecodable, RustcEncodable)]
struct SendMailCredentials {
    access_token: String,
    refresh_token: String,
    expires_in: i64,
    client_id: String,
    client_secret: String,
}

impl SendMailCredentials {

    fn new(access_token: &str, refresh_token: &str, expires_in: i64, client_id: &str, client_secret: &str) -> SendMailCredentials {
        SendMailCredentials {
            access_token: access_token.to_string(),
            refresh_token: refresh_token.to_string(),
            expires_in: expires_in,
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string()
        }
    }

    fn default_new() -> SendMailCredentials {
        SendMailCredentials::new("", "", 0, "", "")
    }

    fn read_from_config() -> Option<SendMailCredentials> {
        let mut r = File::open("credentials.json");
        if r.is_err() {
            return None;
        }
        let mut f = r.unwrap();
        let mut content: String = "".to_string();
        f.read_to_string(&mut content);
        let mut r = json::decode(&content);
        if r.is_err() {
            return None;
        }
        Some(r.unwrap())
    }

    fn write_to_config(&self) -> Result<(),()> {
        let mut f = try!(OpenOptions::new()
                         .create(true)
                         .write(true)
                         .open("credentials.json").map_err(|e|()));
        let encoded = try!(json::encode(self).map_err(|e|()));
        try!(f.write_all(encoded.as_bytes()).map_err(|e|()));
        try!(f.sync_all().map_err(|e|()));
        Ok(())
    }
}

struct SendMailOptions {
    setup: bool,
    credentials: SendMailCredentials,
    from_field: String,
    to_field: String,
    subject_field: String,
    body_field: String
}

fn print_usage(program: &String, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn make_options() -> Options {
    let mut result = Options::new();
    result.optflag("", "setup", "Setup the credentials through OAuth2")
        .optflag("", "help", "Print this usage")
        .optopt("", "client_id", "The client id", "ID")
        .optopt("", "client_secret", "The client secret", "SECRET")
        .optopt("", "from", "From email", "FROM")
        .optopt("", "to", "To email", "TO")
        .optopt("", "subject", "The email subject", "SUBJECT")
        .optopt("", "body", "The email body", "BODY");
    result
}

fn parse_options() -> Result<SendMailOptions, SendMailError> {
    let args: Vec<String> = env::args().collect();
    let options = try!(make_options().parse(args).map_err(|e|SendMailError::Parse));
    let credentials = SendMailCredentials {
            access_token: "".to_string(),
            refresh_token: "".to_string(),
            expires_in: 0,
            client_id: options.opt_default("client_id", "").unwrap(),
            client_secret: options.opt_default("client_secret", "").unwrap()
    };
    let result = SendMailOptions {
        setup: options.opt_present("setup"),
        credentials: credentials,
        from_field: options.opt_default("from", "").unwrap(),
        to_field: options.opt_default("to", "").unwrap(),
        body_field: options.opt_default("body", "").unwrap(),
        subject_field: options.opt_default("subject", "").unwrap()
    };

    Ok(result)
}

fn setup_access_token(client_id: String, client_secret: String) {
    let flow = OAuth2Flow {
        client_id: client_id,
        client_secret: client_secret,
        scopes: vec!("https://www.googleapis.com/auth/gmail.send".to_string()),
        redirect_uri: "urn:ietf:wg:oauth:2.0:oob".to_string(),
        auth_uri: "https://accounts.google.com/o/oauth2/auth".to_string(),
        token_uri: "https://accounts.google.com/o/oauth2/token".to_string()
    };

    let user_grant_url = flow.step_1_get_authorize_url();
    println!("Open your browser at the following url for obtaining an access code");
    println!("{}", user_grant_url);
    println!("Paste the obtained access code here:");
}

fn main() {
    let options = match parse_options() {
        Ok(k) => k,
        Err(_) => return
    };

    if options.setup {
        setup_access_token(options.credentials.client_id,
                           options.credentials.client_secret)
    }
}
