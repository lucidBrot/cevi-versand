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
    const PLACEHOLDER_API_TOKEN : &'static str = "api_token";
    const PLACEHOLDER_LOGIN_EMAIL : &'static str = "login_email";

    fn versand_endpoint_sorted_by_address(&self) -> String{
        self.versand_endpoint_sorted_by_address_fmtstr
            .replace(DB_Conf::PLACEHOLDER_LOGIN_EMAIL, &self.login_email)
            .replace(DB_Conf::PLACEHOLDER_API_TOKEN, &self.api_token)
    }
}

fn get_data_sorted_by_address (db_conf : &DB_Conf) -> Result<(), reqwest::Error> {
    let body = reqwest::get(&db_conf.versand_endpoint_sorted_by_address())?
    .text()?;
    println!("body = {:?}", body);
    println!("link = {}", db_conf.versand_endpoint_sorted_by_address());
    Ok(())
}
