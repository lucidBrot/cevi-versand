/// related to [issue 5](https://github.com/lucidBrot/cevi-versand/issues/5)  
///
/// An interface that allows internal functions to inform the user about something
pub trait UserInteractor {
    fn on_download_finished(&self) {}
    fn on_parsing_finished(&self) {}
    fn report_bad_address(&self, _broken_person: &dbparse::ReasonablePerson) {}
    fn on_pdf_generation_finished(&self) {}
    fn error_missing_config_file(&self, _filename: String) {}
    fn error_injecting_couverts(&self, _error: &dyn std::error::Error) {}
    fn error_fetching_auth_token(&self, _error: &std::io::Error) {}
    fn interactively_get_auth_token(&self) -> Result<AuthTokenData, std::io::Error>;
    fn inform_user(&self, msg: &str) {
        println!("{}", msg);
    }
}

/// Simplistic default user interface
pub struct CliUi {}

impl UserInteractor for CliUi {
    fn on_download_finished(&self) {
        println!("UI: Download Finished.");
    }

    fn on_parsing_finished(&self) {
        println!("UI: Parsing Finished.");
    }

    fn report_bad_address(&self, broken_person: &dbparse::ReasonablePerson) {
        println!(
            "UI: Broken Address Found:
                 {:?}",
            broken_person
        );
    }

    fn on_pdf_generation_finished(&self) {
        println!("UI: Finished generating pdf");
    }

    fn error_missing_config_file(&self, filename: String) {
        println!(
            "UI: File {} was missing. There should now be a template for you to fill in. Do that, then try again.",
            filename
        );
    }

    fn error_injecting_couverts(&self, error: &dyn std::error::Error) {
        println!(
            r###"\
UI: There was an error while trying to inject additional people:
  {:?}
  Perhaps there's a problem with inject_people.yaml?
"###,
            error
        );
    }

    fn error_fetching_auth_token(&self, error: &std::io::Error) {
        println!("Problem fetching token: {:?}", error);
        match error.kind() {
            std::io::ErrorKind::Other => println!("Perhaps your internet connection is down?"),
            std::io::ErrorKind::InvalidData => {
                println!("Perhaps the credentials were wrong, perhaps the database is down, ...")
            },
            _ => (),
        }
    }

    fn interactively_get_auth_token(&self) -> Result<AuthTokenData, std::io::Error> {
        use std::io::Write;
        print!("e-mail: ");
        std::io::stdout().flush()?;
        let mut input_email = String::new();
        let _bytes_read = std::io::stdin()
            .read_line(&mut input_email)
            .expect("Non-utf8 string input!");
        let input_email = input_email.trim();

        print!("Type a pass: ");
        std::io::stdout().flush()?;
        let mut pass = String::new();
        let _bytes_read = std::io::stdin()
            .read_line(&mut pass)
            .expect("Non-utf8 string input for password!");
        let pass = pass.trim();
        // TODO: how to read password without displaying it?
        std::io::stdout().flush()?;

        let auth_token = dbparse::get_auth_token(input_email.as_ref(), pass.as_ref());
        match auth_token {
            Err(e) => {
                self.error_fetching_auth_token(&e);
                Err(e)
            },
            Ok(token) => {
                // TODO: trim will be a problem if the password contains whitespace
                println!("Auth Token: {:?}", token);
                Ok(AuthTokenData {
                    login_email: input_email.to_string(),
                    user_token: token.to_string(),
                })
            },
        }
    }
}

pub struct AuthTokenData {
    pub login_email: String,
    pub user_token: String,
}
