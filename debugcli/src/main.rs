extern crate clap;
use clap::*;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(
    version = "1.0",
    author = "LucidBrot",
    name = "name",
    about = "about"
)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    /// A subcommand for controlling testing
    #[clap(name = "test", version = "1.3", author = "Someone Else", about = "about text")]
    Test(Test),
}

#[derive(Clap)]
struct Test {
    /// Print debug info
    #[clap(short = "d")]
    debug: bool
}

fn main() {
    let opts: Opts = Opts::parse();

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    match opts.subcmd {
        SubCommand::Test(t) => (),
    }

    // more program logic goes here...
}
