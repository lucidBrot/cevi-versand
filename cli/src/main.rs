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
    /// Removes configuration files and replaces them with an empty template.
    #[clap(version = "1.0.0")]
    clean(CleanSubcommand),
    /// Runs the whole chain and sets up template files for any configuration files that are
    /// missing.
    #[clap(version = "1.0.0")]
    run(RunSubcommand),
    /// Helps you set up the config.yaml file to some extent. You WILL have to manually edit it
    /// though, for adding the endpoints. So might as well do everything manually.
    #[clap(version = "1.0.0")]
    setup(SetupSubcommand),
}

/// A subcommand for cleaning files
#[derive(Clap)]
struct CleanSubcommand {
    /// Testrun, Only show what would be removed
    #[clap(short = "r", long = "not-test-run")]
    not_test_run: bool,

    /// Remove also the required files, not just the optional mappings
    #[clap(short = "a", long = "remove-all")]
    remove_required: bool,
}

#[derive(Clap)]
struct RunSubcommand {
    /// Disables the printing of the flag-like sidebadges.
    #[clap(short = "S", long = "disable-sidebadges", conflicts_with = "enable_sidebadges")]
    disable_sidebadges: bool,

    /// Enables the printing of the flag-like sidebadges. (default)
    // Only there for nicer usage, does not do anything, because it is the default in
    // combine::PrintingParameters anyways
    #[clap(short = "s", long = "enable-sidebadges", conflicts_with = "disable_sidebadges")]
    #[allow(dead_code)]
    enable_sidebadges: bool,

    /// Disables the printing of the topside groups
    #[clap(short = "G", long = "disable-groups")]
    disable_groups: bool,

    /// enables the printing of the topside groups
    // Another default that does not do anything except tht it allows explicit specification of the
    // action because it makes -G error when -g is specified.
    #[clap(short = "g", long = "enable-groups", conflicts_with = "disable_groups")]
    #[allow(dead_code)]
    enable_groups: bool,

    /// Disables the printing of the topside (nick-)names
    #[clap(short = "N", long = "disable-nicknames")]
    disable_nicknames: bool,

    /// enables the printing of the topside (nick-)names
    // Another default that does not do anything except tht it allows explicit specification of the
    // action because it makes -N error when -n is specified.
    #[clap(short = "n", long = "enable-nicknames", conflicts_with = "disable_nicknames")]
    #[allow(dead_code)]
    enable_nicknames: bool,

}

#[derive(Clap)]
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
        SubCommand::run(c) => {
            ui.inform_user("Running...");

            combine::main(&ui, &combine::PrintingParameters::new().print_sidebadges(!c.disable_sidebadges).print_groups(!c.disable_groups).print_names(!c.disable_nicknames));

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
