// TODO: error messages e.g. when no internet
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;
pub mod mapping;
use mapping::GroupMapping;

// config.yaml is stored both in examples dir and in dbparse dir, currently. Because it is read
// from the working dir

pub const MAPPING_YAML_FILE: &str = "mapping.yaml";
const VERBOSITY: Verbosity = Verbosity::No;

pub const CONFIG_YAML_FILE: &str = "config.yaml";
/// used for generating the template.
/// Don't confuse this with the placeholder that is supposed to be used within the template links
/// which is inserted at runtime.
const PLACEHOLDER_API_TOKEN: &str = "{the_api_token}";
const PLACEHOLDER_LOGIN_EMAIL: &str = "{the_login_email}";
const PLACEHOLDER_SERVICE_TOKEN: &str = "{the_service_token}";
const CONFIG_YAML_FILLABLE_TEMPLATE: &str = r###"db_conf:
    # --- SECURE LOGIN ---
    # Das service-token muss manuell eingerichtet werden, z.B. unter db.cevi.ch/groups/115/service_tokens
    #    ( Ersetze die Zahl 115 durch die entsprechende Gruppe, der alle endpoint Gruppen untergeordnet sind )
    # Dieses service-token benötigt die Permissions "Personen von Untergruppen"
    # Falls das service_token gesetzt ist, kann in den ENDPOINTS service_token als placeholder verwendet werden.
    service_token: "{the_service_token}"
    # --- USERTOKEN LOGIN ---
    # Das user-token kann automatisch geholt werden. Das ist der einzige Vorteil davon. Dafür ist es weniger
    # sicher, weil es für den ganzen Nutzer das selbe ist, egal für welche Anwendung.
    # Das user-token wird hier auch api-token genannt.
    api_token: "{the_api_token}"
    # die e-mail adresse zum einloggen in der db.cevi.ch
    login_email: "{the_login_email}"
    # 
    # --- ENDPOINTS ---
    # Der link zu den Leuten in der datenbank. Relevant für dich als user sind nur die Zahlen für die gruppen,
    # sowie die filter_id
    # Ersetze sie durch die gruppen-id und filter-id, die du verwenden möchtest.
    # Bei login-type SECURE sind die links generell von der Form
    #    https://db.cevi.ch/groups/2423/people.json?token=[api_token]
    # nur mit geschweiften Klammern {} statt eckigen Klammern [].
    # Bei login-type USERTOKEN sind die links generell von der Form
    #    https://db.cevi.ch/groups/2423/people.json?user_email=[login_email]&user_token=[api_token]
    # nur mit geschweiften Klammern {} statt eckigen Klammern [].
    versand_endpoint_fmtstrs:
        - "https://db.cevi.ch/groups/2423/people.json?user_email={login_email}&user_token={api_token}"
        - "https://db.cevi.ch/groups/116/people.json?filter_id=319&user_email={login_email}&user_token={api_token}"
"###;

const SIGNIN_POST_URL: &str = "https://db.cevi.ch/users/sign_in.json";

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

#[cfg(not(target_arch = "wasm32"))]
pub fn run(user_interface: &dyn DbparseInteractor) -> Result<MainReturns, Box<dyn Error>> {
    // load database API token
    let config = setup_config(user_interface);
    let dataset: ReasonableDataset = get_data_for_versand(&config).expect("WTF in main!");
    user_interface.on_download_finished();
    return run_with_reasonable_dataset(dataset);
}

pub fn run_with_reasonable_dataset(
    dataset: ReasonableDataset,
) -> Result<MainReturns, Box<dyn Error>> {
    // load yaml mapping from file if exists
    let yaml_group_mapping: Result<String, std::io::Error> = read_to_string(MAPPING_YAML_FILE);
    // combine with new groups from database
    let loaded_group_mapping: GroupMapping = match yaml_group_mapping {
        Ok(mapping) => {
            mapping::create_map_from_yaml(&mapping).expect("Creating map from yaml failed")
        },
        Err(e) => {
            println!("problem loading yaml mapping: {}.\nRecreating it...", e);
            GroupMapping::new()
        },
    };
    // create mapping from Database
    let db_group_mapping: GroupMapping = GroupMapping::from_set(&dataset.groups);
    // merge mappings
    let merged_group_mapping: GroupMapping =
        mapping::store_map_in_map(&loaded_group_mapping, &db_group_mapping);
    // save new mapping to file
    let new_yaml_group_mapping: String = mapping::create_yaml_from_map(&merged_group_mapping)
        .expect("Generating yaml for group mapping failed");
    let mut file = File::create(MAPPING_YAML_FILE).expect("Writing mapping failed");
    let res = file.write_all(new_yaml_group_mapping.as_bytes());

    return match res {
        Ok(_) => Ok(MainReturns {
            file: file,
            group_mapping: merged_group_mapping,
            dataset: dataset,
        }),
        Err(e) => Err(Box::new(e)),
    };
}

fn setup_config(ui: &dyn DbparseInteractor) -> DB_Conf {
    let filename = CONFIG_YAML_FILE;
    let fil = match fs::File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            let _result = generate_template_config_file(
                "generisch@cevi.ch",
                "th1s1sY0ur70k3n",
                "th1s1sY0ur53rvic370k3n",
            );
            ui.error_missing_config_file(filename.to_string());
            panic!("failed to find {}: {:?}", filename, e);
        },
    };
    let yaml: serde_yaml::Value = serde_yaml::from_reader(fil).expect("file should be proper YAML");

    let db_conf_in_yaml: &serde_yaml::Value = yaml.get("db_conf").unwrap();
    let db_conf: DB_Conf = serde_yaml::from_value(db_conf_in_yaml.clone()).unwrap();
    println!("deserialized = {:?}", db_conf);
    return db_conf;
}

/// specify the text that should be in the file as placeholders until the user edits it
fn generate_template_config_file_at(
    filename: String,
    api_token_placeholder: &str,
    login_email_placeholder: &str,
    service_token_placeholder: &str,
) -> Result<(), std::io::Error> {
    let mut file = File::create(filename)?;
    file.write_all(
        CONFIG_YAML_FILLABLE_TEMPLATE
            .replace(PLACEHOLDER_API_TOKEN, api_token_placeholder)
            .replace(PLACEHOLDER_LOGIN_EMAIL, login_email_placeholder)
            .replace(PLACEHOLDER_SERVICE_TOKEN, service_token_placeholder)
            .as_bytes(),
    )?;
    println!("generated config.yaml template - please fill it in");
    Ok(())
}

fn generate_template_config_file(
    login_email: &str,
    api_token: &str,
    service_token: &str,
) -> Result<(), std::io::Error> {
    generate_template_config_file_at(
        CONFIG_YAML_FILE.to_string(),
        api_token,
        login_email,
        service_token,
    )
}

/// Wrapper for generate_template_config_file. Sets up config.yaml with your auth token, given your
/// password. Does so by calling get_auth_token
fn generate_config_file(login_email: &str, password: &str) -> Result<(), std::io::Error> {
    let auth_token = get_auth_token(login_email, password)?;
    generate_template_config_file(login_email, auth_token.as_ref(), "th1s1sY0ur53rvic370k3n")
}

fn get_auth_token_url_data(login_email: &str, password: &str) -> String {
    let data = r#"person[email]={email}&person[password]={password}"#;

    data.replace("{email}", login_email)
        .replace("{password}", password)
        .to_string()
}

pub fn get_auth_token(login_email: &str, password: &str) -> Result<String, std::io::Error> {
    use std::io::ErrorKind;
    let data: String = get_auth_token_url_data(login_email, password);
    let body = chttp::post(SIGNIN_POST_URL, data)?.into_body().text()?;

    let yaml: serde_yaml::Value = serde_yaml::from_str(body.as_ref()).unwrap();
    let auth_token: &serde_yaml::Value = yaml
        .get("people")
        .ok_or(std::io::Error::new(
            ErrorKind::InvalidData,
            "People not found",
        ))?
        .get(0)
        .ok_or(std::io::Error::new(
            ErrorKind::InvalidData,
            "No people found",
        ))?
        .get("authentication_token")
        .ok_or(std::io::Error::new(
            ErrorKind::InvalidData,
            "Auth token not found",
        ))?;

    let auth_token_str = auth_token.as_str().ok_or(std::io::Error::new(
        ErrorKind::InvalidData,
        "Auth token not a string",
    ))?;
    return Ok(auth_token_str.to_string());
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
/// the `login_email`/`api_token` combination which might soon be deprecated by the database team
/// in favor of the `service_token` (23.12.2019)
///
/// If any of these is not in use, its content is ignored but must still exist. So I recommend the
/// empty string.
struct DB_Conf {
    api_token: String,
    login_email: String,
    versand_endpoint_fmtstrs: Vec<String>,
    service_token: String,
}
impl DB_Conf {
    // used in yaml to be filled in at runtime
    const PLACEHOLDER_API_TOKEN: &'static str = "{api_token}";
    const PLACEHOLDER_LOGIN_EMAIL: &'static str = "{login_email}";
    const PLACEHOLDER_SERVICE_TOKEN: &'static str = "{service_token}";

    fn format_versand_endpoint(&self, s: String) -> String {
        s.replace(DB_Conf::PLACEHOLDER_LOGIN_EMAIL, &self.login_email)
            .replace(DB_Conf::PLACEHOLDER_API_TOKEN, &self.api_token)
            .replace(DB_Conf::PLACEHOLDER_SERVICE_TOKEN, &self.service_token)
    }

    fn versand_endpoints(&self) -> impl Iterator<Item = String> + '_ {
        self.versand_endpoint_fmtstrs
            .iter()
            .map(move |s| self.format_versand_endpoint(s.to_string()))
            .into_iter()
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn get_data_for_versand(
    db_conf: &DB_Conf,
) -> Result<ReasonableDataset, Box<dyn std::error::Error>> {
    let mut endpoints = db_conf.versand_endpoints();
    let endpoint = endpoints.next();
    if endpoint.is_none() {
        return Err(Box::new(std::io::Error::from(std::io::ErrorKind::Other)));
    }

    let endpoint = endpoint.unwrap();
    let body = chttp::get(endpoint)?.into_body().text()?;
    let mut reasonable_dataset = reasonablify_body(&body)?;
    for endpoint in endpoints {
        let body = chttp::get(endpoint)?.into_body().text()?;
        let reasonable_ds = reasonablify_body(&body)?;
        reasonable_dataset.extend(&reasonable_ds);
    }

    return Ok(reasonable_dataset);
}

fn reasonablify_body(body: &String) -> Result<ReasonableDataset, Box<dyn std::error::Error>> {
    // deserialize the json data into a struct
    let dese: PeopleRequest = serde_json::from_str::<PeopleRequest>(body.as_ref())
        .expect("dbparse: The request response is not well-formatted.");

    let mut i = 0;

    if VERBOSITY.value() >= Verbosity::Much.value() {
        for role in dese.linked.roles_map.iter() {
            println!("Roles[{}] = {:?}", i, role);
            i += 1;
        }
    }

    // transform the Person into a ReasonablePerson, which directly contains all relevant data
    let reasonable_dataset: ReasonableDataset = dese.to_reasonable_dataset();
    if reasonable_dataset.people.len() < 1 {
        panic!("There are no people in the dataset!");
    }

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
///"nickname": "...",
///"company_name": "",
///"company": false,
///"email": "...",
///"address": "...",
///"zip_code": "...",
///"town": "...",
///"country": "...",
///"picture": "https://db.cevi.ch/assets/profil-3a8452c9ac8e8b1b70b9d4f4250417bea5be8a4518dbfae44db944f8fda07ca5.png",
///"salutation_parents": "Herr",
///"name_parents": "..., ...",
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
    pub name: String,       // Gruppenname
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
    pub fn new(
        id: Rc<str>,
        role_type: String,
        label: Option<String>,
        group_id: String,
        layer_group: String,
    ) -> Self {
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
    pub fn gett(&self, s: String) -> Option<&V> {
        return self.get(&*s);
    }
    pub fn gettt(&self, s: &String) -> Option<&V> {
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
    use serde::de::{Deserialize, Deserializer};
    use serde::ser::Serializer;
    use std::rc::Rc;

    pub fn serialize<S>(map: &StringHashMap<Role>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(map.values())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<StringHashMap<Role>, D::Error>
    where
        D: Deserializer<'de>,
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
    use serde::de::{Deserialize, Deserializer};
    use serde::ser::Serializer;
    use std::collections::HashSet;

    pub fn serialize<S>(set: &HashSet<Group>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(set.iter())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<HashSet<Group>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut set = HashSet::<Group>::new();
        for item in Vec::<Group>::deserialize(deserializer)? {
            set.insert(item);
        }
        Ok(set)
    }
}

/// a deserializer implementation for turning null into an empty string
mod null_str_serder {
    use serde::de::{Deserialize, Deserializer};
    use serde::ser::Serializer;

    pub fn serialize<S>(stringthing: &String, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(stringthing)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let optstr = Option::<String>::deserialize(deserializer)?;
        match optstr {
            None => Ok(String::from("")),
            Some(s) => Ok(s),
        }
    }
}

#[derive(Eq, Debug, PartialEq, Clone, Hash)]
pub struct ReasonableGroup {
    pub inner_group: Group,
}
impl From<Group> for ReasonableGroup {
    fn from(g: Group) -> Self {
        ReasonableGroup { inner_group: g }
    }
}
#[derive(Clone, Debug)]
pub struct ReasonableDataset {
    pub people: Vec<ReasonablePerson>,
    groups: HashSet<ReasonableGroup>,
}
impl ReasonableDataset {
    fn get_groups(&self) -> &HashSet<ReasonableGroup> {
        &self.groups
    }

    /// ADDS people from new dataset, does not perform any checks whether they are already included
    fn extend(&mut self, other: &Self) {
        self.people.extend(other.people.clone());
        self.groups = self.groups.union(other.get_groups()).cloned().collect();
    }
}
// to get reasonable information, we want the group that is stored in Role:links, which is found
// by id which we get from Person:links

#[derive(Debug, Clone)]
pub struct ReasonablePerson {
    pub first_name: String,
    pub last_name: String,
    pub nickname: String,
    pub address: String,
    pub zip_code: String,
    pub town: String,
    pub name_parents: String,
    pub roles: HashSet<Role>,
    pub groups: HashSet<ReasonableGroup>,
}
impl PeopleRequest {
    fn to_reasonable_dataset(&self) -> ReasonableDataset {
        let mut all_groups: HashSet<ReasonableGroup> = HashSet::new();
        let mut all_people: Vec<ReasonablePerson> = Vec::<ReasonablePerson>::new();
        if self.people.len() < 1 {
            panic!("There are no people in the dataset!");
        }

        print!("---\n");
        for p in self.people.iter() {
            let mut reasonable_person = ReasonablePerson {
                first_name: p.first_name.trim().to_string(),
                last_name: p.last_name.trim().to_string(),
                nickname: p.nickname.trim().to_string(),
                address: p.address.trim().to_string(),
                zip_code: p.zip_code.trim().to_string(),
                town: p.town.trim().to_string(),
                name_parents: p.name_parents.trim().to_string(),
                roles: HashSet::<Role>::new(),
                groups: HashSet::<ReasonableGroup>::new(),
            };

            // get roles directly
            for role_id in p.links.roles.iter() {
                //let strx: String = as_string(role_id);
                let role: &Role = self
                    .linked
                    .roles_map
                    .gettt(role_id)
                    .expect(&format!("role_id = {} does not exist", role_id));
                reasonable_person.roles.insert(role.clone());

                // get group corresponding to role (linked in Role links) (This could be optimized)
                let group: Group = self
                    .linked
                    .groups
                    .iter()
                    .find(|&grp| grp.id == role.links.group_id)
                    .expect(&format!(
                        "Group with id {} does not exist!",
                        role.links.group_id
                    ))
                    .clone();
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

pub trait DbparseInteractor {
    fn on_download_finished(&self);
    fn error_missing_config_file(&self, filename: String);
}
