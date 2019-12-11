use pdfgen::{CouvertInfo, Receiver, Role};

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
                    role: Role::Custom("Debug".to_string()),
                },
            ],

        }
    ];
    println!("Injected {} people after merging", drug.len());
    println!("As yaml:\n{:?}\n", serde_yaml::to_string(&drug).unwrap());

    couvert_infos.append(&mut drug);
}
