use pdfgen::{CouvertInfo, Receiver, Role};
use std::fs::{OpenOptions, File};
use std::io::{Write, Read};

fn serialize_couvert_infos ( yaml_text : &str ) {
    let c_i_list : Result<Vec<CouvertInfo>, serde_yaml::Error> = serde_yaml::from_str(yaml_text);
    match c_i_list {
        Ok(rec) => println!("success"),
        Err(e) => println!("failure"),
    };
}

/// Reads from `inject_people.yaml` and adds those persons to the parameter `couvert_infos`
pub fn inject_couvert_infos(couvert_infos: &mut Vec<CouvertInfo>) {
    let mut fil = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .truncate(false)
        .open("inject_people.yaml")
        .expect("Creating file inject_people.yaml failed");
    let mut text = String::new();
    dbg!(fil.read_to_string(&mut text));
    let content_result: Result<Vec<CouvertInfo>, serde_yaml::Error> = serde_yaml::from_str(&text);
    match content_result {
        Ok(mut content) => {
            println!("success");
            couvert_infos.append(&mut content);
        },
        Err(e) => println!("Parsing failed: {:?}", e),
    };


    
}
