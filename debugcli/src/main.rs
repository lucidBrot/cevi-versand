extern crate clap;
use clap::*;

#[derive(Clap)]
struct Opts {
    /// Print debug info
    #[clap(short = "d", conflicts_with = "not_debug")]
    debug: bool,
    /// Explicitly disable debug printing
    #[clap(short = "D")]
    not_debug: bool,

}

fn main() {
    let t: Opts = Opts::parse();

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
            if t.debug { println!("testing with debug output on."); }
            if t.not_debug { println!("testing with debug output off."); }
            if t.debug && t.not_debug { println!("WTF!"); }
            if !t.debug && !t.not_debug { println!("Defaulting to testing with debug output off"); }
    }
