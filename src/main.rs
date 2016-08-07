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
    if let Err(err) = settings {
        parser.print_usage(format!("{}", err));
        return;
    }
    let settings = settings.unwrap();
    let credentials = match settings.setup {
        true => Credentials::setup(settings.client_id, settings.client_secret),
        false => Credentials::load_and_refresh()
    };
    if credentials.is_err() {
        println!("Failed to obtain valid credentials. Please rerun with --setup");
        return;
    };
    let credentials = credentials.unwrap();
    let sender = MailSender::new(&credentials);
    sender.send(settings.from_field.as_str(), settings.to_field.as_str(),
                settings.subject_field.as_str(), settings.body_field.as_str());
}
