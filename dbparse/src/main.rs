use serde::{Serialize, Deserialize};
use serde_json;
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
    setup_config();
}

fn setup_config(){
    let file = fs::File::open("config.yaml")
        .expect("file should open read only");
    let yaml: serde_yaml::Value = serde_yaml::from_reader(file)
        .expect("file should be proper YAML");
    let first_name = yaml.get("debug_first_name")
        .expect("file should have FirstName key");
    if let serde_yaml::Value::String(name) = first_name {
        println!("FirstName: {}", name);
    }
}