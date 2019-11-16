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


fn merge_households<'b>( people: &mut 'b Vec<dbparse::ReasonablePerson>,
                     mapping: &dbparse::mapping::GroupMapping) -> Vec<pdfgen::CouvertInfo<'b>> {
    assert_gt!(people.len(), 0);

    // normalize entries in each person so that we can sort
    for person in people.iter_mut() {
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

        person.address = normalize_address(person.address);
        person.town = normalize_town(person.town);
    }

    // sort people be zip, town, last name
    people.sort_by(|a, b|
                   a.zip_code.cmp(b.zip_code)
                   .then(a.town.cmp(b.town))
                   .then(a.last_name.cmp(b.last_name))
                   );

    // look for people that live in the same place
    let couvert_infos : Vec<pdfgen::CouvertInfo> = Vec::with_capacity(people.len());
    let latest_person_considered: &dbparse::ReasonablePerson = &people.get(0).unwrap();
    let couvert_info : pdfgen::CouvertInfo = CouvertInfo {
        receivers: Vec::<Receiver>::new(),
        address: Vec::<&str>::new(),
    }
    couvert_info.
    for person in people.iter().skip(1) {
        
    }

    /*
     receivers: Vec<Receiver>
     address: Vec<&'a str>
    */
    let couvert_info : pdfgen::CouvertInfo = CouvertInfo {
        receivers 
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

fn get_address(person: &dbparse::ReasonablePerson) -> Vec<&str> {
    vec![
        format!("Familie {}", person.last_name),
        person.address,
        format!("{} {}", person.zip_code, person.town),
    ]
}