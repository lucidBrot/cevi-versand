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
    let mut dataset: dbparse::ReasonableDataset = ret_db.dataset;

    println!("combine: merging households");
    // TODO: fn group mapping -> fix "strasse, str, ..."
    // TODO: fn group mapping -> couvert info
    merge_households( &mut dataset.people, &mapping );

    println!("combine: creating pdf");
    pdfgen::main();
    // TODO: generate_couverts instead of main
}


fn merge_households<'b>( people: &'b mut Vec<dbparse::ReasonablePerson>,
                     mapping: &dbparse::mapping::GroupMapping) -> Vec<pdfgen::CouvertInfo<'b>> {
    assert!(people.len() > 0);

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

        person.address = normalize_address(&person.address);
        person.town = normalize_town(&person.town);
    }

    // sort people be zip, town, last name
    people.sort_by(|a, b|
                   a.zip_code.cmp(&b.zip_code)
                   .then(a.town.cmp(&b.town))
                   .then(a.last_name.cmp(&b.last_name))
                   );

    // look for people that live in the same place
    let mut couvert_infos : Vec<pdfgen::CouvertInfo> = Vec::with_capacity(people.len());
    let first_person: &dbparse::ReasonablePerson = &people.get(0).unwrap();
    let mut couvert_info : pdfgen::CouvertInfo = pdfgen::CouvertInfo {
        receivers: Vec::<pdfgen::Receiver>::new(),
        address: get_address(first_person),
    };
    couvert_info.receivers.push(into_receiver(first_person));
    couvert_infos.push(couvert_info);

    for person in people.iter().skip(1) {
        let addr = get_address(person);
        let previous_addr = &couvert_infos.last().unwrap().address;
        let receiver = into_receiver(person);

        if addr == *previous_addr {
            // add to previous couvert another receiver
            couvert_infos.last_mut().unwrap().receivers.push(receiver);
        } else {
            let mut couvert_info : pdfgen::CouvertInfo = pdfgen::CouvertInfo {
                receivers: Vec::<pdfgen::Receiver>::new(),
                address: addr,
            };
            couvert_info.receivers.push(receiver);
            couvert_infos.push(couvert_info);
        }
    }

    return couvert_infos;
}

/// removes newlines within address
/// trims starting and ending whitespace
/// replaces "str."  with "strasse" and replaces " str." with "Strasse"
/// ```
/// let addr : String = String::from(" add\nressstr.  ");
/// let normalized : String = normalize_address(addr);
/// assert_eq!(normalized, String::from("addressstrasse"))
/// ```
fn normalize_address(address: &String) -> String {
    return String::from("Herbert"); // TODO: remove this
}

/// replaces Pfäffikon, Pfaeffikon, etc with "Pfäffikon ZH"
/// TODO: documentation test
fn normalize_town(town: &String) -> String {
    return String::from("Herbert"); // TODO: obviously wrong. does the test fail?
}

fn get_address(person: &dbparse::ReasonablePerson) -> Vec<&str> {
    vec![
        &format!("Familie {}", person.last_name),
        &*person.address,
        &*format!("{} {}", person.zip_code, person.town),
    ]
}

fn into_receiver(person: &dbparse::ReasonablePerson) -> pdfgen::Receiver {
    pdfgen::Receiver {
        nickname: person.nickname.clone(),
        group: person.groups.iter().nth(0).expect(&*format!("Person has no group: {:?}", person)).inner_group.name.clone() , // TODO: get best role
        role: role_convert(&person.roles.iter().nth(0).expect(&*format!("Person has no role: {:?}", person))), // TODO: get best group
    }
}

/// This function has not yet really been thought about.
/// It is unimplemented for all purposes
/// ```
/// assert!(false, "PLIZ IMPLIMENT");
/// ```
fn role_convert(role_dpbarse : &dbparse::Role) -> pdfgen::Role {
   // TODO: somehow figure out the role based on the ugly dbparse role
   return pdfgen::Role::Traegerkreis;
}

