// TODO: error messages e.g. when no internet
use serde::{Serialize, Deserialize};
use std::fs;
use std::collections::HashMap;
use std::rc::Rc;
use std::collections::HashSet;
mod mapping;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // load database API token
    let config = setup_config();
    let dataset : ReasonableDataset = get_data_sorted_by_address(&config).expect("WTF in main!");
    mapping::create_yaml_from_set(&dataset.groups);
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

fn get_data_sorted_by_address (db_conf : &DB_Conf) -> Result<ReasonableDataset, reqwest::Error> {
    let body : String = reqwest::get(&db_conf.versand_endpoint_sorted_by_address())?
    .text()?;
    // deserialize the json data into a struct
    let dese: PeopleRequest = serde_json::from_str::<PeopleRequest>(&body).expect("I am bad");

    let mut i = 0;
    for role in dese.linked.roles_map.iter() {
        println!("Roles[{}] = {:?}",i,  role);
        i+=1;
    }

    // transform the Person into a ReasonablePerson, which directly contains all relevant data
    let reasonable_dataset : ReasonableDataset = dese.to_reasonable_dataset();

    Ok(reasonable_dataset)
}

#[derive(Serialize, Deserialize, Debug)]
struct PeopleRequest {
    people: Vec<Person>,
    linked: Linked,
}

// deserialize "linked" : "roles" : []   as a map
// see https://github.com/serde-rs/serde/issues/936
#[derive(Serialize, Deserialize, Debug)]
struct Linked {
    #[serde(with = "items_serder_set", rename = "groups")]
    groups: HashSet<Group>,
    #[serde(with = "items_serder_map", rename = "roles")]
    roles_map: StringHashMap<Role>, // actual roles in a hashmap
}

/// stored in "people": []
///
/// JSON of a single Person from sorted by address:
/// 
/// ```json
/// "id": "6468",
///"type": "people",
///"href": "https://db.cevi.ch/groups/1334/people/6468.json",
///"first_name": "Eric",
///"last_name": "Mink",
///"nickname": "Levanzo",
///"company_name": "",
///"company": false,
///"email": "eric@mink.li",
///"address": "Neuwiesenstrasse 2",
///"zip_code": "8332",
///"town": "Russikon",
///"country": "CH",
///"picture": "https://db.cevi.ch/assets/profil-3a8452c9ac8e8b1b70b9d4f4250417bea5be8a4518dbfae44db944f8fda07ca5.png",
///"salutation_parents": "Herr",
///"name_parents": "Simon, Yvonne",
///"links": {
/// "ortsgruppe": "115",
///     "phone_numbers": [
///     "8919",
///     "8920"
///     ],
/// "roles": [
///     "37855",
///     "46790",
///     "52789"
/// ]
///}
///```
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
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
struct RoleLinks {
    #[serde(rename = "group")]
    group_id: String,
    layer_group: String,
}

/// stored in "linked" : "groups" : []
/// See [Linked](struct.Linked.html)
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
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
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
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
pub struct Role {
    id: Rc<str>,
    role_type: String,
    label: Option<String>,
    links: RoleLinks,
}

type StringHashMapType<V> = HashMap<Rc<str>, V>;
#[derive(Debug)]
pub struct StringHashMap<V>(StringHashMapType<V>);
/// implement HashMap<Rc<str>, Role>::get() for a String instead of only a &str
/// See https://www.reddit.com/r/rust/comments/2snn7a/hashmaprcstring_v/
impl<V> StringHashMap<V> {
    pub fn gett(&self, s:String) -> Option<&V> {
        return self.get(&*s);
    }
    pub fn gettt(&self, s:&String) -> Option<&V> {
        return self.get(&**s);
    }
    pub fn new() -> Self {
        return StringHashMap(StringHashMapType::<V>::new());
    }
    pub fn insertt(&mut self, k: String, v: V) -> Option<V> {
        return self.insert(Rc::from(&*k), v);
    }
}
// allow dereferencing to the oldtype to avoid writing &self.0.get()
impl<V> std::ops::Deref for StringHashMap<V> {
    type Target = StringHashMapType<V>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<V> std::ops::DerefMut for StringHashMap<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
/// a serializer/deserializer implementation for turning a list of Role items into a hashmap with the
/// id:String
/// as key
mod items_serder_map {
    use super::Role;
    use super::StringHashMap;
    use serde::ser::Serializer;
    use serde::de::{Deserialize, Deserializer};
    use std::rc::Rc;

    pub fn serialize<S>(map: &StringHashMap<Role>, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.collect_seq(map.values())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<StringHashMap<Role>, D::Error>
        where D: Deserializer<'de>
    {
        let mut map = StringHashMap::<Role>::new();
        for item in Vec::<Role>::deserialize(deserializer)? {
            map.insert(Rc::clone(&item.id), item);
        }
        Ok(map)
    }
}

/// a serializer/deserializer implementation for turning a list of Group items into a hashset
mod items_serder_set {

    use super::Group;
    use std::collections::HashSet;
    use serde::ser::Serializer;
    use serde::de::{Deserialize, Deserializer};

    pub fn serialize<S>(set: &HashSet<Group>, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.collect_seq(set.iter())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<HashSet<Group>, D::Error>
        where D: Deserializer<'de>
    {
        let mut set = HashSet::<Group>::new();
        for item in Vec::<Group>::deserialize(deserializer)? {
            set.insert(item);
        }
        Ok(set)
    }
}

#[derive(Eq, Debug, PartialEq, Clone, Hash)]
pub struct ReasonableGroup{pub inner_group: Group}
impl From<Group> for ReasonableGroup {
    fn from(g: Group) -> Self {
        ReasonableGroup{inner_group: g}
    }
}
pub struct ReasonableDataset {
    people: Vec<ReasonablePerson>,
    groups: HashSet<ReasonableGroup>,
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
    roles: HashSet<Role>,// TODO: set of enums instead?
    groups: HashSet<ReasonableGroup>,
}
impl PeopleRequest {
    fn to_reasonable_dataset(&self) -> ReasonableDataset {
        let mut all_groups : HashSet<ReasonableGroup> = HashSet::new();

        print!("---\n");
        for p in self.people.iter() {
            let mut reasonable_person = ReasonablePerson {
                first_name: p.first_name.clone(),
                last_name: p.last_name.clone(),
                nickname: p.nickname.clone(),
                address: p.address.clone(),
                zip_code: p.zip_code.clone(),
                town: p.town.clone(),
                name_parents: p.name_parents.clone(),
                roles: HashSet::<Role>::new(),
                groups: HashSet::<ReasonableGroup>::new(),
            };

            // get roles directly
            for role_id in p.links.roles.iter() {
                //let strx: String = as_string(role_id);
                let role: &Role = self.linked.roles_map.gettt(role_id).expect(&format!("role_id = {} does not exist", role_id)); 
                reasonable_person.roles.insert(role.clone());

                // get group corresponding to role (linked in Role links) (This could be optimized)
                let group: Group = self.linked.groups.iter().find(|&grp| grp.id == role.links.group_id).expect(&format!("Group with id {} does not exist!", role.links.group_id)).clone();
                let reasonable_group: ReasonableGroup = group.into();
                reasonable_person.groups.insert(reasonable_group.clone()); 
                // store group if it appeared at least once also at the top level of the dataset
                all_groups.insert(reasonable_group);
            }
        }
        print!("\n");
        print!("all_groups = {:#?}", all_groups);

        ReasonableDataset {
            people: Vec::<ReasonablePerson>::new(),
            groups: all_groups,
        }
    }
}
