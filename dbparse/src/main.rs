use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);

    // load database API token
    let config = setup_config();
    get_data_sorted_by_address(&config);
}

fn setup_config() -> DB_Conf {
    let fil = fs::File::open("config.yaml")
        .expect("file should open read only");
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
    println!("body = {:?}", body);
    // TODO: deserialize, into what?
    let deserialized: serde_json::Value = serde_json::from_str(&body).unwrap();
    let people: Vec<Person> = serde_json::from_value(deserialized.get("people").unwrap().clone()).unwrap();
    Ok(body)
}

/*
 * JSON from sorted by address:
 *
  "people":[ 
      { 
         "id":"18389",
         "type":"people",
         "href":"https://db.cevi.ch/groups/1341/people/18389.json",
         "first_name":"Leon Jonas",
         "last_name":"Roth",
         "nickname":"Takka",
         "company_name":"",
         "company":false,
         "email":"pfarramt.roth@gmail.com",
         "address":"Am Burenbüel 28",
         "zip_code":"8320",
         "town":"Fehraltorf",
         "country":"CH",
         "picture":"https://db.cevi.ch/assets/profil-3a8452c9ac8e8b1b70b9d4f4250417bea5be8a4518dbfae44db944f8fda07ca5.png",
         "salutation_parents":"",
         "name_parents":"Barbara und Martin",
         "links":{ 
            "ortsgruppe":"115",
            "phone_numbers":[ 
               "22487",
               "28203",
               "28204"
            ],
            "roles":[ 
               "54716"
            ]
         }
 *
 */
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    id: usize,
    href: String,
    first_name: String,
    last_name: String,
    nickname: String,
    email: String,
    address: String,
    zip_code: String,
    town: String,
    name_parents: String,
    roles: Vec<usize>,
}
