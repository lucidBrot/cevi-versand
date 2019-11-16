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


fn merge_households<'b>( people: &'b Vec<dbparse::ReasonablePerson>,
                     mapping: &dbparse::mapping::GroupMapping) -> Vec<pdfgen::CouvertInfo<'b>> {
    // TODO: first replace strasse and str
    // sort by (dorf, street, last name ....)
    // merge same into one couvert info
    // store single ones into one couvert info
    // return couvert infos
    for person in people {
        /* Person
        first_name: String
        last_name: String
        nickname: String
        address: String
        zip_code: String
        town: String
        name_parents: String
        roles: HashSet<Role>
        groups: HashSet<ReasonableGroup>*/

        let normalized_address = normalize_address(person.address);
    }

}

/// removes newlines within address
/// trims starting and ending whitespace
/// replaces "str."  with "strasse" and replaces " str." with "Strasse"
/// ```
/// let addr : String = String::from(" add\nressstr.  ");
/// let normalized : String = normalize_address(addr);
/// assert_eq!(normalized, String::from("addressstrasse"))
/// ```
fn normalize_address(address: String) -> String {
    
}

/// replaces Pfäffikon, Pfaeffikon, etc with "Pfäffikon ZH"
/// TODO: documentation test
fn normalize_town(town: String) -> String {

}
