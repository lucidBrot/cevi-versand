// TODO: error messages e.g. when no internet
use serde::{Serialize, Deserialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::fs::read_to_string;
use std::collections::HashMap;
use std::rc::Rc;
use std::collections::HashSet;
use std::error::Error;
pub mod mapping;
use mapping::GroupMapping;

// config.yaml is stored both in examples dir and in dbparse dir, currently. Because it is read
// from the working dir

// TODO: set nickname to first name in couvert

const MAPPING_YAML_FILE : &str = "mapping.yaml";
const VERBOSITY : Verbosity = Verbosity::No;

pub enum Verbosity {
    No,
    ABit,
    Much,
}
impl Verbosity {
    fn value(&self) -> u8 {
        match self {
            Verbosity::No => 0,
            Verbosity::ABit => 5,
            Verbosity::Much => 10,
        }
    }
}

pub struct MainReturns {
    pub file: File,
    pub group_mapping: GroupMapping,
    pub dataset: ReasonableDataset,
}

fn main() {
    run().expect("Unexpected");
}

pub fn run() -> Result<MainReturns, Box<dyn Error>> {
    // load database API token
    let config = setup_config();
    let dataset : ReasonableDataset = get_data_for_versand(&config).expect("WTF in main!");

    // load yaml mapping from file if exists
    let yaml_group_mapping : Result<String, std::io::Error> = read_to_string(MAPPING_YAML_FILE);
    // combine with new groups from database
    let loaded_group_mapping : GroupMapping =
        match yaml_group_mapping {
            Ok(mapping) => mapping::create_map_from_yaml(&mapping).expect("Creating map from yaml failed"),
            Err(e) => { println!("problem loading yaml mapping: {}.\nRecreating it...", e); GroupMapping::new() },
        };
    // create mapping from Database
    let db_group_mapping : GroupMapping = GroupMapping::from_set(&dataset.groups);
    // merge mappings
    let merged_group_mapping : GroupMapping = mapping::store_map_in_map(&loaded_group_mapping, &db_group_mapping);
    // save new mapping to file
    let new_yaml_group_mapping : String = mapping::create_yaml_from_map(&merged_group_mapping).expect("Generating yaml for group mapping failed");
    let mut file = File::create(MAPPING_YAML_FILE).expect("Writing mapping failed");
    let res = file.write_all(new_yaml_group_mapping.as_bytes());
    
    return match res {
        Ok(_) => Ok(MainReturns {
            file: file,
            group_mapping: merged_group_mapping,
            dataset: dataset,
        }),
        Err(e) => Err(Box::new(e)),
    }
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
struct DB_Conf { // TODO: accept DB_Conf as parameter
    login_name: String,
    api_token: String,
    login_email: String,
    versand_endpoint_fmtstr: String,
}
impl DB_Conf {
    // used in yaml to be filled in at runtime
    const PLACEHOLDER_API_TOKEN : &'static str = "{api_token}";
    const PLACEHOLDER_LOGIN_EMAIL : &'static str = "{login_email}";

    fn versand_endpoint(&self) -> String{
        self.versand_endpoint_fmtstr
            .replace(DB_Conf::PLACEHOLDER_LOGIN_EMAIL, &self.login_email)
            .replace(DB_Conf::PLACEHOLDER_API_TOKEN, &self.api_token)
    }
}

fn get_data_for_versand (db_conf : &DB_Conf) -> Result<ReasonableDataset, reqwest::Error> {
    let body : String = reqwest::get(&db_conf.versand_endpoint())?
    .text()?;
    // deserialize the json data into a struct
    let dese: PeopleRequest = serde_json::from_str::<PeopleRequest>(&body).expect("dbparse: The request response is not well-formatted.");

    let mut i = 0;

    if VERBOSITY.value() >= Verbosity::Much.value() {
        for role in dese.linked.roles_map.iter() {
            println!("Roles[{}] = {:?}",i,  role);
            i+=1;
        }
    }

    // transform the Person into a ReasonablePerson, which directly contains all relevant data
    let reasonable_dataset : ReasonableDataset = dese.to_reasonable_dataset();
    if reasonable_dataset.people.len() < 1 {panic!("There are no people in the dataset!");}

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
    #[serde(with = "null_str_serder")]
    href: String,
    #[serde(with = "null_str_serder")]
    first_name: String,
    #[serde(with = "null_str_serder")]
    last_name: String,
    #[serde(with = "null_str_serder")]
    nickname: String,
    #[serde(with = "null_str_serder")]
    address: String,
    #[serde(with = "null_str_serder")]
    zip_code: String,
    #[serde(with = "null_str_serder")]
    town: String,
    #[serde(with = "null_str_serder")]
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
pub struct RoleLinks {
    #[serde(rename = "group")]
    pub group_id: String,
    pub layer_group: String,
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
    pub id: String,
    pub name: String, // Gruppenname
    pub group_type: String, // Ortsgruppe/Untergruppe/Mitglieder/Jungschar/Verein...
}

/// `role_type` can be things like "Teilnehmer/-in", "Gruppenleiter/-in", "Minigruppenleiter/-in", "Mitglied",
/// "Adressverwalter/-in", "Hausverantwortliche/-r", "Adressverantwortlicher", ...
///
/// When it is "Gruppenleiter/-in", the `label` might be set to "Stufenleiterin", "Stufenleiter",
/// or "Stufenleiter/-in"
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
pub struct Role {
    id: Rc<str>,
    pub role_type: String,
    label: Option<String>,
    links: RoleLinks,
}
impl Role {
    pub fn new(id: Rc<str>, role_type: String, label: Option<String>, group_id: String, layer_group: String ) -> Self{
        Role {
            id: id,
            role_type: role_type,
            label: label,
            links: RoleLinks {
                group_id: group_id,
                layer_group: layer_group,
            },
        }
    }
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

/// ad deserializer implementation for turning null into an empty string
mod null_str_serder {
    use serde::ser::Serializer;
    use serde::de::{Deserialize, Deserializer};
    
    pub fn serialize<S>(stringthing: &String, serializer: S) -> Result<S::Ok, S::Error> 
        where S: Serializer 
    {
        serializer.serialize_str(stringthing)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
        where D: Deserializer<'de>
    {
        let optstr = Option::<String>::deserialize(deserializer)?;
        match optstr {
            None => Ok(String::from("")),
            Some(s) => Ok(s),
        }
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
    pub people: Vec<ReasonablePerson>,
    groups: HashSet<ReasonableGroup>,
}
// to get reasonable information, we want the group that is stored in Role:links, which is found
// by id which we get from Person:links

#[derive(Debug)]
pub struct ReasonablePerson {
    pub first_name: String,
    pub last_name: String,
    pub nickname: String,
    pub address: String,
    pub zip_code: String,
    pub town: String,
    pub name_parents: String,
    pub roles: HashSet<Role>,// TODO: set of enums instead?
    pub groups: HashSet<ReasonableGroup>,
}
impl PeopleRequest {
    fn to_reasonable_dataset(&self) -> ReasonableDataset {
        let mut all_groups : HashSet<ReasonableGroup> = HashSet::new();
        let mut all_people : Vec<ReasonablePerson> = Vec::<ReasonablePerson>::new();
        if self.people.len() < 1 {panic!("There are no people in the dataset!");}

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

            all_people.push(reasonable_person);
        }

        print!("\n");
        if VERBOSITY.value() >= Verbosity::Much.value() {
            print!("all_groups = {:#?}", all_groups);
        }

        ReasonableDataset {
            people: all_people,
            groups: all_groups,
        }
    }
}
