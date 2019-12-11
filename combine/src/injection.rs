use pdfgen::{CouvertInfo, Receiver, Role};
use serde::{Serialize, Deserialize};

/// Implementing Serialize for a stranger's type. See https://serde.rs/remote-derive.html
#[derive(Serialize, Deserialize)]
#[serde(remote = "CouvertInfo")]
struct CouvertInfoDef {
    receivers: Vec<Receiver>,
    address: Vec<String>,
}

/// Implementing Serialize for a stranger's type. See https://serde.rs/remote-derive.html
#[derive(Serialize, Deserialize)]
#[serde(remote = "Receiver")]
struct ReceiverDef {
    nickname: String,
    group: String,
    role: Role,
}

fn serialize_couvert_infos ( text : String ) {
    let mut deserializer = serde_json::Deserializer::from_str(text);
    let rec_result = ReceiverDef::deserialize(&mut deserializer);
    match rec_result {
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

    couvert_infos.append(&mut drug);
}
