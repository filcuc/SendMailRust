extern crate rustc_serialize;
extern crate url;
extern crate getopts;
extern crate hyper;
mod oauth2_flow;
mod settings_parser;
use settings_parser::*;
mod credentials;
use credentials::*;
mod mail_sender;
use mail_sender::*;

fn main() {
    let parser = SettingsParser::new();
    let settings = parser.parse();
    if settings.is_err() {
        let brief = format!("{}", settings.err().unwrap());
        parser.print_usage(brief);
        return;
    }
    let mut settings = settings.unwrap();
    settings.client_id = "259740745275-uqaacpq45uak4avaciepv8u3ffgv286k.apps.googleusercontent.com".to_string();
    settings.client_secret = "DM6Kp3fF_0_ANVIJWYQUxI4n".to_string();
    let credentials = match settings.setup {
        true => Credentials::setup(settings.client_id, settings.client_secret),
        false => Credentials::load()
    };

    MailSender::send()
}
