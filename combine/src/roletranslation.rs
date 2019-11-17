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
    
    return pdfgen::Role::Traegerkreis;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_role_to_role_sample() {
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
}
