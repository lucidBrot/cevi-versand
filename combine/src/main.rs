use pdfgen;
use dbparse;

fn main() {
    println!("combine: loading data from database");
    let database_returns: Result<dbparse::MainReturns, Box<dyn std::error::Error>> = dbparse::run();
    if database_returns.is_err() {
        std::process::exit(1);
    }
    let ret_db: dbparse::MainReturns = database_returns.unwrap();
    let mapping: dbparse::mapping::GroupMapping = ret_db.group_mapping;
    let dataset: dbparse::ReasonableDataset = ret_db.dataset;

    println!("combine: merging households");
    // TODO: fn group mapping -> fix "strasse, str, ..."
    // TODO: fn group mapping -> couvert info
    merge_households( &dataset.people, &mapping );

    println!("combine: creating pdf");
    pdfgen::main();
    // TODO: generate_couverts instead of main
}


fn merge_households( people: &Vec<dbparse::ReasonablePerson>,
                     mapping: &dbparse::mapping::GroupMapping) {
    // TODO: first replace strasse and str
    // sort by (dorf, street, last name ....)
    // merge same into one couvert info
    // store single ones into one couvert info
    // return couvert infos
    
}
