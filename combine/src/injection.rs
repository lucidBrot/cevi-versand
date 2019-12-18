use pdfgen::CouvertInfo;
use std::fs::OpenOptions;
use std::io::{Write, Read};

pub const INJECTION_YAML_FILE_PATH: &str = "inject_people.yaml";
const INJECTION_YAML_FILE_TEMPLATE: &str = r###"---
# remove the following line (or comment it out):
[]
# and replace it with something like this:
# - receivers:
#     - nickname: Herbert
#       group: Herbert Fan Club
#       role: Teilnehmer
#   address:
#     - Herbert Herber
#     - Herbertstrasse h32
#     - 8332 Herbhausen
# - receivers:
#     - nickname: Zweibert
#       group: Herbert Fan Club
#       role: Teilnehmer
#   address:
#     - Zweibert Herber
#     - Herbertstrasse h32
#     - 8332 Herbhausen
# - receivers:
#     - nickname: HERBERT
#       group: Herbert Fan Club
#       role: Teilnehmer
#   address:
#     - Herbert Herber
#     - Herbertstrasse h32
#     - 8332 Herbhausen

# Each receiver will get their own envelope. But the envelopes will be sorted like all the other envelopes, by group name.
# This File will be regenerated if you delete it
"###;

// mostly for debug purposes
#[allow(dead_code)]
fn serialize_couvert_infos(yaml_text: &str) {
    let c_i_list: Result<Vec<CouvertInfo>, serde_yaml::Error> = serde_yaml::from_str(yaml_text);
    match c_i_list {
        Ok(_rec) => println!("success"),
        Err(e) => println!("failure: {:?}", e),
    };
}

/// Reads from `inject_people.yaml` and adds those persons to the parameter `couvert_infos`
pub fn inject_couvert_infos(
    couvert_infos: &mut Vec<CouvertInfo>,
    user_interface: &dyn ui::UserInteractor,
) {
    // create empty-ish template file iff there is no current file there
    let fi = OpenOptions::new()
        .write(true)
        .read(true)
        .create_new(true)
        .open(INJECTION_YAML_FILE_PATH);
    let fil = match fi {
        Err(_) => {
            // if failed to create a new file, it was already there. That's good.
            OpenOptions::new()
                .write(true)
                .read(true)
                .create(true)
                .truncate(false)
                .open(INJECTION_YAML_FILE_PATH)
        }
        // if successfully created a new file, it is empty
        Ok(mut f) => {
            match f.write_all(INJECTION_YAML_FILE_TEMPLATE.as_bytes()) {
                Ok(()) => Ok(f),
                Err(e) => Err(e),
            }
        }
    };

    match fil {
        Err(e) => {
            dbg!("Creating/Opening file {} failed", INJECTION_YAML_FILE_PATH);
            user_interface.error_injecting_couverts(&e);
            return;
        }
        Ok(_) => (),
    };
    let mut file = fil.unwrap();

    dbg!(&file);
    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Ok(_success_code) => (),
        Err(error) => {
            dbg!(
                "An error occurred while reading {}: {:?}",
                INJECTION_YAML_FILE_PATH,
                error.kind()
            );
            user_interface.error_injecting_couverts(&error);
        }
    };

    let content_result: Result<Vec<CouvertInfo>, serde_yaml::Error> = serde_yaml::from_str(&text);
    match content_result {
        Ok(mut content) => {
            couvert_infos.append(&mut content);
        }
        Err(e) => {
            println!("Parsing failed: {:?}", e);
            user_interface.error_injecting_couverts(&e);
        }
    };
}
