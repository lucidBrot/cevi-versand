use pdfgen::CouvertInfo;
use std::fs::OpenOptions;
use std::io::{Read, Write};

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

pub fn create_injection_yaml_file_template() -> Result<(), std::io::Error> {
    let mut fi = create_injection_yaml_file_empty()?;
    // if new file created, write template string to it
    fi.write_all(INJECTION_YAML_FILE_TEMPLATE.as_bytes())?;
    Ok(())
}

pub fn create_injection_yaml_file_empty() -> Result<std::fs::File, std::io::Error> {
    OpenOptions::new()
        .write(true)
        .read(true)
        .create_new(true)
        .open(INJECTION_YAML_FILE_PATH)
}

/// Reads from `inject_people.yaml` and adds those persons to the parameter `couvert_infos`
pub fn inject_couvert_infos(
    mut couvert_infos: &mut Vec<CouvertInfo>,
    user_interface: &dyn ui::UserInteractor,
) {
    // create empty-ish template file iff there is no current file there
    let fi = create_injection_yaml_file_empty();

    if let Err(_) = fi {
        // if failed to create a new file, it was already there. That's good.
        let fil = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(false)
            .open(INJECTION_YAML_FILE_PATH);

        match fil {
            Err(e) => {
                println!(
                    "combine::inject: Failed to open r/w file {}",
                    INJECTION_YAML_FILE_PATH
                );
                user_interface.error_injecting_couverts(&e);
                return;
            },
            Ok(mut file) => {
                let mut text = String::new();
                match file.read_to_string(&mut text) {
                    Err(error) => {
                        println!(
                            "combine::inject: Failed to read file {}",
                            INJECTION_YAML_FILE_PATH
                        );
                        user_interface.error_injecting_couverts(&error);
                    },
                    Ok(_success_code) => {
                        parse_and_append(&text, couvert_infos, user_interface);
                    },
                }
            },
        }
    } else {
        // if new file created, write template string to it
        let mut f = fi.unwrap();
        let res = f.write_all(INJECTION_YAML_FILE_TEMPLATE.as_bytes());
        if let Err(e) = res {
            println!("combine::inject: Failed to write template file.");
            user_interface.error_injecting_couverts(&e);
        }

        // and use template string as text
        parse_and_append(
            &INJECTION_YAML_FILE_TEMPLATE,
            &mut couvert_infos,
            user_interface,
        );
    };
}

fn parse_and_append(
    text: &str,
    couvert_infos: &mut Vec<CouvertInfo>,
    user_interface: &dyn ui::UserInteractor,
) {
    let content_result: Result<Vec<CouvertInfo>, serde_yaml::Error> = serde_yaml::from_str(text);
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
