#[macro_use]
extern crate clap;
use clap::*;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = "1.0", author = "Eric Mink v/o Levanzo", name = "cevi-versand", about = "cevi-versand generiert Couverts, warnt vor fehlenden infos, kombiniert Personen die zusammen wohnen in einen einzelnen Umschlag, und fügt relevante Informationen für das Versandteam hinzu.")]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short = "c", long = "config", default_value = "default.conf")]
    config: String,
    /// Some input. Because this isn't an Option<T> it's required to be used
    input: String,
    /// A level of verbosity, and can be used multiple times
    #[clap(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: i32,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
#[allow(non_camel_case_types)]
enum SubCommand {
    clean(CleanSubcommand),
}

/// A subcommand for cleaning files
#[derive(Clap)]
struct CleanSubcommand {
    /// Testrun, Only show what would be removed
    #[clap(short = "n", long = "test-run" )]
    test_run: bool,
}

fn main() {
    let opts: Opts = Opts::parse();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    println!("Value for config: {}", opts.config);
    println!("Using input file: {}", opts.input);

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match opts.verbose {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    match opts.subcmd {
        SubCommand::clean(t) => {
            if t.test_run {
                println!("Printing debug info...");
            } else {
                println!("Printing normally...");
            }
        },
    }

    // more program logic goes here...
}
