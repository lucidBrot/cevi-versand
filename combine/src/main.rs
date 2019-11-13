use pdfgen;
use dbparse;

fn main() {
    println!("combine: loading data from database");
    let database_returns: Result<dbparse::MainReturns, Box<dyn std::error::Error>> = dbparse::run();
    if database_returns.is_err() {
        std::process::exit(1);
    }
    let mapping: dbparse::mapping::GroupMapping = database_returns.unwrap().group_mapping;

    println!("combine: creating pdf");
    pdfgen::main();
}
