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
pub fn role_to_role(dbrole: &dbparse::Role) -> pdfgen::Role{

    match dbrole.role_type.as_ref() { // TODO: test with whole database
        "Teilnehmer/-in" => pdfgen::Role::Teilnehmer,
        "Traegerkreis" => pdfgen::Role::Traegerkreis, // TODO: how to recognize trägerkreis?
        "Minigruppenleiter/-in" => pdfgen::Role::Leiter,
        // TODO: case where it did not match
        &_ => println!("r2r: don't know what to do with {}", _),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_role_to_role_teilnehmer() {
        use std::rc::Rc;

        let db_role = dbparse::Role::new(
            /*id: Rc<str>,
            pub role_type: String,
            label: Option<String>,
            links: RoleLinks,*/
            std::rc::Rc::<str>::from("test_id"),
            String::from("Teilnehmer"),
            None,
            String::from("Fake group id"),
            String::from("fake layer group"),
        );
        
        let pdf_role: pdfgen::Role = super::role_to_role(&db_role);

        assert_eq!(pdf_role, pdfgen::Role::Teilnehmer);
    }

    #[test]
    fn test_role_to_role_unexpected() {
        use std::rc::Rc;

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

        assert_eq!(pdf_role, pdfgen::Role::Teilnehmer);
    }

    #[test]
    fn test_role_to_role_traegerkreis() {
        assert!(false, "please implement");
    }
}
