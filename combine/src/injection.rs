use pdfgen::{CouvertInfo};
use std::fs::{OpenOptions};
use std::io::{Read};

pub const INJECTION_YAML_FILE_PATH: &str = "inject_people.yaml";

// mostly for debug purposes
#[allow(dead_code)]
fn serialize_couvert_infos ( yaml_text : &str ) {
    let c_i_list : Result<Vec<CouvertInfo>, serde_yaml::Error> = serde_yaml::from_str(yaml_text);
    match c_i_list {
        Ok(_rec) => println!("success"),
        Err(e) => println!("failure: {:?}",e),
    };
}

/// Reads from `inject_people.yaml` and adds those persons to the parameter `couvert_infos`
pub fn inject_couvert_infos(couvert_infos: &mut Vec<CouvertInfo>, user_interface: &dyn ui::UserInteractor) {
    let fil = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .truncate(false)
        .open(INJECTION_YAML_FILE_PATH);
    match fil {
        Err(e) => {
            dbg!("Creating file {} failed", INJECTION_YAML_FILE_PATH);
            user_interface.error_injecting_couverts(&e);
            return;
        },
        Ok(_) => (),
    };
    let mut file = fil.unwrap();

    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Ok(_success_code) => (),
        Err(error) => {
            dbg!("An error occurred while reading {}: {:?}", INJECTION_YAML_FILE_PATH, error.kind());
            user_interface.error_injecting_couverts(&error);

        },
    };

    let content_result: Result<Vec<CouvertInfo>, serde_yaml::Error> = serde_yaml::from_str(&text);
    match content_result {
        Ok(mut content) => {
            couvert_infos.append(&mut content);
        },
        Err(e) => {
            println!("Parsing failed: {:?}", e);
            user_interface.error_injecting_couverts(&e);
        },
    };
 
}
