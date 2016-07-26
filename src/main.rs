mod oauth2_flow;
mod option_parser;
mod credentials;
mod mail_sender;

fn main() {
    let parser = SettingsParser::new();
    let settings = parser.parse();
    if settings.is_err() {
        let brief = format!("{}", settings.err().unwrap());
        parser.print_usage(brief);
        return;
    }
    let settings = settings.unwrap();
    let credentials = match settings.setup {
        true => Credentials::setup(settings.client_id, settings.client_secret),
        false => Credentials::load()
    };

    // MailSender::send()
}
