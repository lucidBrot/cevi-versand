use dbparse;
use pdfgen;
use regex;
mod roletranslation;

// TODO: warn about adresses where the address is incomplete

pub fn main() {
    println!("combine: loading data from database");
    let database_returns: Result<dbparse::MainReturns, Box<dyn std::error::Error>> = dbparse::run();
    if database_returns.is_err() {
        std::process::exit(1);
    }
    let ret_db: dbparse::MainReturns = database_returns.unwrap();
    let mapping: dbparse::mapping::GroupMapping = ret_db.group_mapping;
    let mut dataset: dbparse::ReasonableDataset = ret_db.dataset;

    println!("combine: merging households");
    let mut couvert_infos: Vec<pdfgen::CouvertInfo> = merge_households(&mut dataset.people, &mapping);

    println!("combine: creating pdf");
    let filename = "output_versand.pdf";
    let doc_generated = pdfgen::generate_couverts(&mut couvert_infos);
    let mut outfile =
        std::io::BufWriter::new(std::fs::File::create(filename).expect("Failed to create file..."));
    doc_generated
        .save(&mut outfile)
        .expect("Failed to save file...");
}

fn merge_households<'b>(
    people: &'b mut Vec<dbparse::ReasonablePerson>,
    mapping: &dbparse::mapping::GroupMapping,
) -> Vec<pdfgen::CouvertInfo> {
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
        warn_if_address_incomplete(&person);
        person.town = normalize_town(&person.town);
    }

    // sort people be zip, town, last name
    people.sort_by(|a, b| {
        a.zip_code
            .cmp(&b.zip_code)
            .then(a.town.cmp(&b.town))
            .then(a.last_name.cmp(&b.last_name))
    });

    // look for people that live in the same place
    let mut couvert_infos: Vec<pdfgen::CouvertInfo> = Vec::with_capacity(people.len());
    let first_person: &dbparse::ReasonablePerson = &people.get(0).unwrap();
    let mut couvert_info: pdfgen::CouvertInfo = pdfgen::CouvertInfo {
        receivers: Vec::<pdfgen::Receiver>::new(),
        address: get_address(first_person, /*use family:*/ false),
    };
    couvert_info
        .receivers
        .push(into_receiver(first_person, &mapping));
    couvert_infos.push(couvert_info);
    let mut previous_family_address = get_address(first_person, false);

    for person in people.iter().skip(1) {
        let addr_family = get_address(person, true);
        let receiver = into_receiver(person, &mapping);

        if addr_family == previous_family_address {
            // add to previous couvert another receiver
            couvert_infos.last_mut().unwrap().receivers.push(receiver);
            couvert_infos.last_mut().unwrap().address = addr_family;
        } else {
            let mut couvert_info: pdfgen::CouvertInfo = pdfgen::CouvertInfo {
                receivers: Vec::<pdfgen::Receiver>::new(),
                address: get_address(person, /*family:*/ false),
            };
            couvert_info.receivers.push(receiver);
            couvert_infos.push(couvert_info);

            previous_family_address = addr_family.clone();
        }
    }

    return couvert_infos;
}

/// removes newlines within address
/// trims starting and ending whitespace
/// replaces "str."  with "strasse" and replaces " str." with "Strasse"
/// ```
/// # use combine::normalize_address;
/// let addr : String = String::from(" add\nressstr.  ");
/// let normalized : String = normalize_address(&addr);
/// assert_eq!(normalized, String::from("addressstrasse"))
/// ```
pub fn normalize_address(address: &String) -> String {
    let trimmed = address.trim().replace("\n", "").replace("\r", "");
    let rgx_str1 = regex::Regex::new(r"str\.").unwrap();
    let rgx_str2 = regex::Regex::new(r"\sstr.").unwrap();
    let replaced1 = rgx_str1.replace_all(&trimmed, "strasse");
    let replaced2 = rgx_str2.replace_all(&replaced1, "Strasse");
    return String::from(replaced2);
}

fn warn_if_address_incomplete(person: &dbparse::ReasonablePerson) -> bool{

    let issue: bool = person.first_name.is_empty() || 
                        person.last_name.is_empty() ||
                        person.address.is_empty() ||
                        person.zip_code.is_empty() ||
                        person.town.is_empty();

    if issue {
        println!("combine: WARN: Bad Address:\n{:?}\n\n", person);
    }

    return issue;
}

/// replaces Pfäffikon, Pfaeffikon, etc with "Pfäffikon ZH"
/// ## testcases
/// ```
/// let town = combine::normalize_town(&String::from("Pfaeffikon "));
/// assert_eq!(String::from("Pfäffikon ZH"), town);
/// ```
///
/// ```
/// # use combine::normalize_town;
/// # let t = &String::from(" pfa\neffikon ZH ");
/// assert_eq!(String::from("Pfäffikon ZH"), normalize_town(t));
/// ```
///
pub fn normalize_town(town: &String) -> String {
    let trimmed = town.trim().replace("\n", "").replace("\r", "");
    let rgx = regex::Regex::new(r"(?i)Pf(ae|ä)ffikon(\s?ZH)?").unwrap();
    return String::from(
        rgx.replace_all(&trimmed, /*replace with:*/ "Pfäffikon ZH")
            .trim(),
    );
}

fn get_address(
    person: &dbparse::ReasonablePerson,
    use_familie_instead_of_first_name: bool,
) -> Vec<String> {
    let first_or_family = if use_familie_instead_of_first_name {
        String::from("Familie")
    } else {
        person.first_name.clone()
    };
    vec![
        format!("{} {}", first_or_family, person.last_name),
        person.address.clone(),
        format!("{} {}", person.zip_code, person.town),
    ]
}

fn into_receiver(
    person: &dbparse::ReasonablePerson,
    group_mapping: &dbparse::mapping::GroupMapping,
) -> pdfgen::Receiver {

    let pdfgen_roles = person.roles.iter().map(|x| roletranslation::role_to_role(x));
    let best_pdfgen_role: pdfgen::Role = pdfgen_roles.max_by_key(|x| x.priority()).unwrap_or(pdfgen::Role::Nothing);

    let best_group_perhaps: Option<&dbparse::ReasonableGroup> = person.groups.iter().max_by_key(|x| x.priority());
    let display_name = match best_group_perhaps {
        Some(group) => group_mapping.get_display_name(&group.inner_group.id).expect("Group id does not exist. Something is messed up."),
        None => String::from(""),
    };

    // if nickname is empty, use first name
    let name = match person.nickname.trim().is_empty() {
        true => person.first_name.clone(),
        false => person.nickname.clone(),
    };

    pdfgen::Receiver {
        nickname: name,
        group: display_name,
        role: best_pdfgen_role, 
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Priority ( i32 );
trait Prioritized {
    fn priority(&self) -> Priority;
}
impl Prioritized for pdfgen::Role {
    /// Higher priority is assigned to pdfgen::Role enum variants that should be preferredly
    /// printed on the couverts
    fn priority(&self) -> Priority {
        use pdfgen::Role;
        match self {
            Role::Traegerkreis => Priority(50),
            Role::Ehemalige => Priority(45),
            Role::Leiter => Priority(40),
            Role::Teilnehmer => Priority(30),
            Role::Nothing => Priority(0),
            _ => Priority(-100), // we don't care whether it's a coach or a Kassier or a Matchef
        }
    }
}

impl Prioritized for dbparse::ReasonableGroup {
    /// Higher priority for more specific groups
    fn priority(&self) -> Priority {
        match self.inner_group.group_type.as_str() {
            "Dachverband" => Priority(10),
            "Mitgliederorganisation" => Priority(30),
            "Sektion" => Priority(40),
            "Verein" => Priority(45),
            "Jungschar" => Priority(49),
            "Gruppe" => Priority(50), // M oder F
            "Ortsgruppe" => Priority(60), // Pfä-Feh-Hi-Rus
            "Stufe" => Priority(70),
            "Mitglieder" => Priority(71),
            //--- end useless stuff ---
            "Externe" => Priority(80), // e.g. "Gebetsgruppe", "C-Newsletter", "Leiter ehemalig", "Ehemalige"
            "Ten-Sing" => Priority(80),
            "Gremium" => Priority(85), // e.g. "C-Gruppe", "Cevi Plus Team"
            "Vorstand" => Priority(90), // if somebody is in a group and in vorstand, we want the group
            "Untergruppe" => Priority(100), // We want always this. It's e.g. Holon.
            "Fröschli" => Priority(100), // Whyever this exists
            // if something is not in this list, we don't want it in almost all cases
            _ => Priority(0),
        }
    }
}

#[cfg(test)]
mod tests {

}
