use serde::{Serialize, Deserialize};
use std::fs;
use std::collections::HashMap;
use std::rc::Rc;
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // load database API token
    let config = setup_config();
    get_data_sorted_by_address(&config);
}

fn setup_config() -> DB_Conf {
    let fil = fs::File::open("config.yaml")
        .expect("config file missing or not readable");
    let yaml: serde_yaml::Value = serde_yaml::from_reader(fil)
        .expect("file should be proper YAML");
    
    let db_conf_in_yaml : &serde_yaml::Value = yaml.get("db_conf").unwrap();
    let db_conf : DB_Conf = serde_yaml::from_value(db_conf_in_yaml.clone()).unwrap();
    println!("deserialized = {:?}", db_conf);
    return db_conf;
}


#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
struct DB_Conf {
    login_name: String,
    api_token: String,
    login_email: String,
    versand_endpoint_sorted_by_address_fmtstr: String,
}
impl DB_Conf {
    // used in yaml to be filled in at runtime
    const PLACEHOLDER_API_TOKEN : &'static str = "{api_token}";
    const PLACEHOLDER_LOGIN_EMAIL : &'static str = "{login_email}";

    fn versand_endpoint_sorted_by_address(&self) -> String{
        self.versand_endpoint_sorted_by_address_fmtstr
            .replace(DB_Conf::PLACEHOLDER_LOGIN_EMAIL, &self.login_email)
            .replace(DB_Conf::PLACEHOLDER_API_TOKEN, &self.api_token)
    }
}

fn get_data_sorted_by_address (db_conf : &DB_Conf) -> Result<String, reqwest::Error> {
    let body : String = reqwest::get(&db_conf.versand_endpoint_sorted_by_address())?
    .text()?;
    // deserialize the json data into a struct
    let dese: PeopleRequest = serde_json::from_str::<PeopleRequest>(&body).expect("I am bad");
    let people: Vec<Person> = dese.people;

    let mut i = 0;
    for role in dese.linked.roles_map.iter() {
        println!("Roles[{}] = {:?}",i,  role);
        i+=1;
    }

    // transform the Person into a ReasonablePerson, which directly contains all relevant data
    let reasonable_people: Vec<ReasonablePerson> = dese.to_reasonable_people();

    Ok(body)
}

/*
 * JSON from sorted by address:
 *
 * "id": "6468",
"type": "people",
"href": "https://db.cevi.ch/groups/1334/people/6468.json",
"first_name": "Eric",
"last_name": "Mink",
"nickname": "Levanzo",
"company_name": "",
"company": false,
"email": "eric@mink.li",
"address": "Neuwiesenstrasse 2",
"zip_code": "8332",
"town": "Russikon",
"country": "CH",
"picture": "https://db.cevi.ch/assets/profil-3a8452c9ac8e8b1b70b9d4f4250417bea5be8a4518dbfae44db944f8fda07ca5.png",
"salutation_parents": "Herr",
"name_parents": "Simon, Yvonne",
"links": {
"ortsgruppe": "115",
"phone_numbers": [
"8919",
"8920"
],
"roles": [
"37855",
"46790",
"52789"
]
}
 *
 */
#[derive(Serialize, Deserialize, Debug)]
struct PeopleRequest {
    people: Vec<Person>,
    linked: Linked,
}

// deserialize "linked" : "roles" : []   as a map
// see https://github.com/serde-rs/serde/issues/936
#[derive(Serialize, Deserialize, Debug)]
struct Linked {
    groups: Vec<Group>,
    #[serde(with = "items_serder", rename = "roles")]
    roles_map: HashMap<Rc<str>, Role>, // actual roles in a hashmap
}

/// stored in "people": []
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    id: usize,
    href: String,
    first_name: String,
    last_name: String,
    nickname: String,
    address: String,
    zip_code: String,
    town: String,
    name_parents: String,
    links: PersonLinks,
}

/// stored within Person struct
#[derive(Serialize, Deserialize, Debug)]
struct PersonLinks {
    roles: Vec<String>, // ids of roles
}

/// stored within Role struct
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
struct RoleLinks {
    group: String,
    layer_group: String,
}

/// stored in "linked" : "groups" : []
///
/// ```name``` contains something like "Aktive", "Holon (M)",
/// "Pfäffikon-Fehraltorf-Hittnau-Russikon", "Verein Pfä...", "Freie Mitarbeiter", "Z_Import
/// Optigem", "Ehemalige", "C-Gruppe", "Gebetsbrunch", "Y-Card Nummer", ...
///
/// ```group_type``` contains something like
/// * "Untergruppe" for Holon
/// * "Ortsgruppe", "Jungschar", "Verein", "Gremium", "Dachverband" for useless groupings
/// * "Vorstand"
/// * "Externe" for J&S stuff, Ehemalige, Freie Mitarbeiter
/// * "Fröschli"
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
pub struct Group {
    id: String,
    name: String, // Gruppenname
    group_type: String, // Ortsgruppe/Untergruppe/Mitglieder/Jungschar/Verein...
}

/// `role_type` can be things like "Teilnehmer/-in", "Gruppenleiter/-in", "Minigruppenleiter/-in", "Mitglied",
/// "Adressverwalter/-in", "Hausverantwortliche/-r", "Adressverantwortlicher", ...
///
/// When it is "Gruppenleiter/-in", the `label` might be set to "Stufenleiterin", "Stufenleiter",
/// or "Stufenleiter/-in"
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
pub struct Role {
    id: Rc<str>,
    role_type: String,
    label: Option<String>,
    links: RoleLinks,
}
/// a serializer/deserializer implementation for turning a list of items into a hashmap with the
/// id:String
/// as key
mod items_serder {
    use super::Role;
    use std::collections::HashMap;
    use serde::ser::Serializer;
    use serde::de::{Deserialize, Deserializer};
    use std::rc::Rc;

    pub fn serialize<S>(map: &HashMap<Rc<str>, Role>, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.collect_seq(map.values())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<Rc<str>, Role>, D::Error>
        where D: Deserializer<'de>
    {
        let mut map = HashMap::new();
        for item in Vec::<Role>::deserialize(deserializer)? {
            map.insert(Rc::clone(&item.id), item);
        }
        Ok(map)
    }
}

// to get reasonable information, we want the group that is stored in Role:links, which is found
// by id which we get from Person:links
#[derive(Debug)]
pub struct ReasonablePerson {
    first_name: String,
    last_name: String,
    nickname: String,
    address: String,
    zip_code: String,
    town: String,
    name_parents: String,
    roles: HashSet<Role>,// TODO: set of enums?
    groups: HashSet<Group>,
}
impl PeopleRequest {
    fn to_reasonable_people(&self) -> Vec<ReasonablePerson> {
        panic!("Not implemented yet");
        for p in self.people {
            let return_val = ReasonablePerson {
                first_name: p.first_name,
                last_name: p.last_name,
                nickname: p.nickname,
                address: p.address,
                zip_code: p.zip_code,
                town: p.town,
                name_parents: p.name_parents,
                roles: HashSet::<Role>::new(),
                groups: HashSet::<Group>::new(),
            };
        }

        return Vec::<ReasonablePerson>::new();
    }
}
