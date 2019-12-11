use pdfgen::{CouvertInfo, Receiver, Role};

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
