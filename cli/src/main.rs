extern crate clap;
use clap::*;
use combine;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(
    version = "1.0",
    author = "Eric Mink v/o Levanzo",
    name = "cevi-versand",
    about = "cevi-versand generiert Couverts, warnt vor fehlenden infos, kombiniert Personen die zusammen wohnen in einen einzelnen Umschlag, und fügt relevante Informationen für das Versandteam hinzu."
)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
#[allow(non_camel_case_types)]
enum SubCommand {
    clean(CleanSubcommand),
    run(RunSubcommand),
    setup(SetupSubcommand),
}

/// A subcommand for cleaning files
#[derive(Clap)]
#[clap(name = "test", version = "1.3", author = "Someone Else", about = "ASDF")]
struct CleanSubcommand {
    /// Testrun, Only show what would be removed
    #[clap(short = "r", long = "not-test-run")]
    not_test_run: bool,

    /// Remove also the required files, not just the optional mappings
    #[clap(short = "a", long = "remove-all")]
    remove_required: bool,
}

#[derive(Clap)]
/// A subcommand for running when config.yaml is set up
struct RunSubcommand {}

#[derive(Clap)]
/// A subcommand that helps you set up the config.yaml file
struct SetupSubcommand {
    #[clap(short = "e", long = "email")]
    email: Option<String>,
    #[clap(short = "p", long = "password")]
    password: Option<String>,
    #[clap(short = "t", long = "service-token")]
    service_token: Option<String>,
}

fn main() {
    let opts: Opts = Opts::parse();

    use ui::UserInteractor;
    let ui = ui::CliUi {};

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    match opts.subcmd {
        SubCommand::clean(t) => {
            if t.not_test_run {
                ui.inform_user("Cleaning For Real...");
            } else {
                ui.inform_user("Cleaning Test Run Started. Re-run with -r to run for real...");
            }
            combine::clean(t.remove_required, !t.not_test_run, Some(&ui)).expect("Failed cleaning. You might need to delete some files manually. Run a test run to see which files all would be removed. Might have failed because those files were not there in the first place.");
        },
        SubCommand::run(_c) => {
            ui.inform_user("Running...");

            combine::main(&ui);

            ui.inform_user("Done. If above output looks problematic - check the output pdf anyway. Perhaps the program fixed everything on its own.");
        },
        SubCommand::setup(s) => {
            let mut email: Option<String> = s.email.clone();
            let api_token = if s.service_token.is_none()
                && (s.email.is_none() || s.password.is_none())
            {
                ui.inform_user("You would be better off running this with command line arguments. Providing a service token there is advised btw. However, I'll now ask you at least for your email and password so that we can get a soon-to-be-deprecated api token.");
                let retval = ui
                    .interactively_get_auth_token()
                    .expect("Failed to get authentication token data. Aborting.");

                email = Some(retval.login_email);
                retval.user_token
            } else if s.email.is_some() && s.password.is_some() {
                dbparse::get_auth_token(
                    s.email.clone().unwrap().as_ref(),
                    s.password.unwrap().as_ref(),
                )
                .expect("Failed to get authentication token. Aborting!")
            } else {
                "".to_string()
            };

            if email.is_none() {
                email = Some("".to_string());
            }

            let service_token = if s.service_token.is_none() {
                "".to_string()
            } else {
                s.service_token.unwrap()
            };

            ui.inform_user("Overwriting config file...");
            dbparse::generate_template_config_file(
                email.unwrap().as_ref(),
                api_token.as_ref(),
                service_token.as_ref(),
            )
            .expect("Something went wrong while generating the config file. Sorry!");
            ui.inform_user("Set Up config file. Open it up, specify your endpoints, then try the subcommand `run`.");
        },
    }

    // more program logic goes here...
}
