use dbparse;
use pdfgen;
// create map of all roles from db
// auto-map to roles from pdfgen (Leiter, Teilnehmer, Traegerkreis, Ehemalig)
// auto-map some roles to be ignored, so that only reasonable roles remain
// create yaml file so user can specify what other roles should map to what, and what roles to
// ignore
//
//
// Or perhaps simply ignore everything that is not obviously
// Leiter/Teilnehmer/Traegerkreis/Ehemalig. No user interaction required.
//
pub fn role_to_role(dbrole: &dbparse::Role) -> pdfgen::Role {
    match dbrole.role_type.as_ref() {
        "Teilnehmer/-in" => pdfgen::Role::Teilnehmer,
        "Traegerkreis" => pdfgen::Role::Traegerkreis, // Trägerkreis is not a role, but a group. Roles would either be "Externe/-r" or "Verantwortliche/-r"
        "Minigruppenleiter/-in" => pdfgen::Role::Leiter,
        "Gruppenleiter/-in" => pdfgen::Role::Leiter,
        "Coach" => pdfgen::Role::Coach,
        "Abteilungsleiter/-in" => pdfgen::Role::Leiter,
        "Adressverwalter/-in" => pdfgen::Role::Nothing,
        "Adressverantwortlicher" => pdfgen::Role::Nothing,
        "Chorsänger/-in" => pdfgen::Role::Nothing,
        "Fröschlihauptleiter/-in" => pdfgen::Role::Leiter,
        "Kassier" => pdfgen::Role::Kassier,
        "Freie/-r Mitarbeiter/-in" => pdfgen::Role::Nothing,
        "Hausverantwortliche/-r" => pdfgen::Role::Hausverantwortlicher,
        "Administrator/-in Cevi DB" => pdfgen::Role::Admin,
        "Externe/-r" => pdfgen::Role::Nothing, // TODO: how to recognize ehemalige?
        "Lädeliverantwortliche/-r" => pdfgen::Role::Laedeli,
        "Mitglied" => pdfgen::Role::Nothing,
        "Stufenleiter/-in" => pdfgen::Role::Leiter,
        "Fröschlileiter/-in" => pdfgen::Role::Leiter,
        "Aktuar/-in" => pdfgen::Role::Aktuar,
        "Materialverantwortliche/-r" => pdfgen::Role::Matchef,
        "Verantwortliche/-r" => pdfgen::Role::Nothing,

        // case where it did not match. That means something new has been added to the DB.
        &_ => {
            println!("r2r: don't know what to do with {:?}", dbrole.role_type);
            return pdfgen::Role::Nothing;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_role_to_role_teilnehmer() {
        let db_role = dbparse::Role::new(
            /*id: Rc<str>,
            pub role_type: String,
            label: Option<String>,
            links: RoleLinks,*/
            std::rc::Rc::<str>::from("test_id"),
            String::from("Teilnehmer/-in"),
            None,
            String::from("Fake group id"),
            String::from("fake layer group"),
        );

        let pdf_role: pdfgen::Role = super::role_to_role(&db_role);

        assert_eq!(pdf_role, pdfgen::Role::Teilnehmer);
    }

    #[test]
    fn test_role_to_role_unexpected() {
        let db_role = dbparse::Role::new(
            /*id: Rc<str>,
            pub role_type: String,
            label: Option<String>,
            links: RoleLinks,*/
            std::rc::Rc::<str>::from("test_id"),
            String::from("etwas unerwartets"),
            None,
            String::from("Fake group id"),
            String::from("fake layer group"),
        );

        let pdf_role: pdfgen::Role = super::role_to_role(&db_role);

        assert_eq!(pdf_role, pdfgen::Role::Nothing);
    }

    #[test]
    fn test_role_to_role_traegerkreis() {
        assert!(false, "please implement");
    }
}
