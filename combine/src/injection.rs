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

pub fn inject_couvert_infos(couvert_infos: &mut Vec<CouvertInfo>) {
    let mut drug: Vec<CouvertInfo> = vec![
        CouvertInfo {
            address: vec![
                "Herbert Herber".to_string(),
                "Herbertstrasse h32".to_string(),
                "8332 Herbhausen".to_string(),
            ],
            receivers: vec![
                Receiver {
                    nickname: "Herbert".to_string(),
                    group: "Herbert Fan Club".to_string(),
                    //role: Role::Custom("Debug".to_string()),
                    role: Role::Teilnehmer,
                },
            ],
        },
        CouvertInfo {
            address: vec![
                "Zweibert Herber".to_string(),
                "Herbertstrasse h32".to_string(),
                "8332 Herbhausen".to_string(),
            ],
            receivers: vec![
                Receiver {
                    nickname: "Zweibert".to_string(),
                    group: "Herbert Fan Club".to_string(),
                    role: Role::Teilnehmer,
                    //role: Role::Custom("Debug".to_string()),
                },
            ],
        }
    ];
    println!("Injected {} people after merging", drug.len());
    println!("As yaml:\n{:?}\n", serde_yaml::to_string(&drug).unwrap());
    let mut fil = File::create("injectedPeople.yaml").unwrap();
    fil.write_all(serde_yaml::to_string(&drug).unwrap().as_bytes()).unwrap();

    let mut fil = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open("inject_people.yaml")
        .expect("Creating file inject_people.yaml failed");
    let mut text = String::new();
    fil.read_to_string(&mut text);
    let content_result: Result<Vec<CouvertInfo>, serde_yaml::Error> = serde_yaml::from_str(&text);
    match content_result {
        Ok(content) => println!("success"),
        Err(e) => println!("Parsing failed: {:?}", e),
    };


    couvert_infos.append(&mut drug);
}
