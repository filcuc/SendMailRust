use getopts::Options;
use std::fmt;
use std::env;
use std::fs::File;

#[derive(Debug)]
pub enum SettingsParserErrorKind {
    GenericError,
    NotEnoughArguments,
    MissingCredentialFileSetupNeeded,
    MissingFromField,
    MissingToField,
    MissingSubjectField,
    MissingBodyField,
    SetupMissingClientId,
    SetupMissingClientSecret,
    ParsingError,
    PrintHelp
}

#[derive(Debug)]
pub struct SettingsParserError {
    kind: SettingsParserErrorKind
}

impl fmt::Display for SettingsParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self.kind {
            SettingsParserErrorKind::GenericError => "Generic Error",
            SettingsParserErrorKind::MissingCredentialFileSetupNeeded => "Missing Credential file Setup Needed",
            SettingsParserErrorKind::MissingFromField => "Missing --from field",
            SettingsParserErrorKind::MissingToField => "Missing --to field",
            SettingsParserErrorKind::MissingSubjectField => "Missing --subject field",
            SettingsParserErrorKind::MissingBodyField => "Missing --body field",
            SettingsParserErrorKind::NotEnoughArguments => "Expected at least an argument",
            SettingsParserErrorKind::SetupMissingClientId => "Setup requested but --client_id is missing",
            SettingsParserErrorKind::SetupMissingClientSecret => "Setup requested but --client_secret is missing",
            SettingsParserErrorKind::ParsingError => "Invalid arguments provided",
            SettingsParserErrorKind::PrintHelp => "Program help requested",
        };
        writeln!(f, "{}", message)
    }
}

pub struct SettingsParser {
    parser: Options
}

pub struct Settings {
    pub setup: bool,
    pub client_id: String,
    pub client_secret: String,
    pub from_field: String,
    pub to_field: String,
    pub subject_field: String,
    pub body_field: String
}

impl SettingsParser {
    pub fn new() -> SettingsParser {
        let parser = {
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
        };
        SettingsParser { parser: parser}
    }

    pub fn print_usage<S>(&self, brief: S) where S: Into<String> {
        let brief = format!("Info: {}", brief.into());
        println!("{}", self.parser.usage(brief.as_str()))
    }

    pub fn parse(&self) -> Result<Settings, SettingsParserError> {
        let args: Vec<String> = env::args().collect();
        if args.len() == 1 {
            return Err(SettingsParserError { kind: SettingsParserErrorKind::NotEnoughArguments })
        }
        let matches = self.parser.parse(&args);
        if matches.is_err() {
            return Err(SettingsParserError { kind: SettingsParserErrorKind::ParsingError })
        }
        let matches = matches.unwrap();
        let settings = Settings {
            setup: matches.opt_present("setup"),
            client_id: matches.opt_str("client_id").unwrap_or(String::new()),
            client_secret: matches.opt_str("client_secret").unwrap_or(String::new()),
            from_field: matches.opt_str("from").unwrap_or(String::new()),
            to_field: matches.opt_str("to").unwrap_or(String::new()),
            subject_field: matches.opt_str("subject").unwrap_or(String::new()),
            body_field: matches.opt_str("body").unwrap_or(String::new())
        };
        if matches.opt_present("help") {
            return Err(SettingsParserError { kind: SettingsParserErrorKind::PrintHelp })
        }
        if matches.opt_present("setup") {
            if !matches.opt_present("client_id") {
                return Err(SettingsParserError { kind: SettingsParserErrorKind::SetupMissingClientId })
            }
            if !matches.opt_present("client_secret") {
                return Err(SettingsParserError { kind: SettingsParserErrorKind::SetupMissingClientSecret })
            }
            return Ok(settings)
        }
        if !matches.opt_present("from") {
            return Err(SettingsParserError { kind: SettingsParserErrorKind::MissingFromField })
        }
        if !matches.opt_present("to") {
            return Err(SettingsParserError { kind: SettingsParserErrorKind::MissingToField })
        }
        if !matches.opt_present("subject") {
            return Err(SettingsParserError { kind: SettingsParserErrorKind::MissingSubjectField })
        }
        if !matches.opt_present("body") {
            return Err(SettingsParserError { kind: SettingsParserErrorKind::MissingBodyField })
        }
        Ok(settings)
    }
}
