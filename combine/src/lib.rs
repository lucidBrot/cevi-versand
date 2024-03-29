use dbparse;
use pdfgen;
use regex;
mod injection;
mod roletranslation;

/// all files that the user might modify to set config
/// used in --info and in --clean
///
/// This is a function instead of a `pub const` because constants cannot allocate a Vec
#[allow(non_snake_case)]
pub fn get_USER_RELEVANT_FILES() -> Vec<&'static dyn AsRef<std::path::Path>> {
    vec![
        &dbparse::MAPPING_YAML_FILE,
        &dbparse::CONFIG_YAML_FILE,
        &crate::injection::INJECTION_YAML_FILE_PATH,
    ]
}

pub fn main_cli_ui() {
    let user_interface = ui::CliUi {};
    let params = PrintingParameters::new().print_sidebadges(true);
    main(&user_interface, &params);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn main(user_interface: &dyn ui::UserInteractor, printing_parameters: &PrintingParameters) {
    let dbparse_interactor = DbparseRedirector {
        user_interface: Some(user_interface),
    };

    println!("combine: loading data from database");
    let database_returns: Result<dbparse::MainReturns, Box<dyn std::error::Error>> =
        dbparse::run(&dbparse_interactor);
    if database_returns.is_err() {
        std::process::exit(1);
    }
    let ret_db: dbparse::MainReturns = database_returns.unwrap();
    let mapping: dbparse::mapping::GroupMapping = ret_db.group_mapping;
    let mut dataset: dbparse::ReasonableDataset = ret_db.dataset;
    user_interface.on_parsing_finished();

    let mut couvert_infos: Vec<pdfgen::CouvertInfo> =
        merge_households(&mut dataset.people, &mapping, user_interface, !printing_parameters.merge_flatmates);
    injection::inject_couvert_infos(&mut couvert_infos, user_interface);
    couvert_infos.sort_by(|a: &pdfgen::CouvertInfo, b: &pdfgen::CouvertInfo| {
        a.receivers[0].group.cmp(&b.receivers[0].group)
    });

    println!("combine: creating pdf");
    let filename = "output_versand.pdf";
    let doc_generated = pdfgen::generate_couverts(
        &mut couvert_infos,
        Some(user_interface),
        printing_parameters.print_sidebadges,
        printing_parameters.print_groups,
        printing_parameters.print_names,
    );
    let mut outfile =
        std::io::BufWriter::new(std::fs::File::create(filename).expect("Failed to create file..."));
    doc_generated
        .save(&mut outfile)
        .expect("Failed to save file...");
}

#[cfg(target_arch = "wasm32")]
pub fn main() {
    println!("combine: main() not implemented for wasm32");
}

/// A builder for cosmetic arguments
pub struct PrintingParameters {
    print_sidebadges: bool,
    print_groups: bool,
    print_names: bool,
    merge_flatmates: bool,
}
impl PrintingParameters {
    pub fn new() -> Self {
        PrintingParameters {
            print_sidebadges : true,
            print_groups: true,
            print_names: true,
            merge_flatmates: true,
        }
    }

    pub fn print_sidebadges(mut self, b: bool) -> Self {
        self.print_sidebadges = b;
        self
    }

    pub fn print_groups(mut self, b: bool) -> Self {
        self.print_groups = b;
        self
    }

    pub fn print_names (mut self, b: bool) -> Self {
        self.print_names = b;
        self
    }

    pub fn merge_flatmates (mut self, b: bool) -> Self {
        self.merge_flatmates = b;
        self
    }
}

fn merge_households<'b>(
    people: &'b mut Vec<dbparse::ReasonablePerson>,
    mapping: &dbparse::mapping::GroupMapping,
    user_interface: &dyn ui::UserInteractor,
    normalize_but_dont_merge: bool,
) -> Vec<pdfgen::CouvertInfo> {
    assert!(people.len() > 0);

    // normalize entries in each person so that we can sort
    for person in people.iter_mut() {
        /* Person
        first_name: String
        last_name: String
        nickname: String
        address: String
        zip_code: String
        town: String
        name_parents: String
        roles: HashSet<Role>
        groups: HashSet<ReasonableGroup>*/

        person.address = normalize_address(&person.address);
        warn_if_address_incomplete(&person, user_interface);
        person.town = normalize_town(&person.town);
    }

    // sort people be zip, town, last name
    people.sort_by(|a, b| {
        a.zip_code
            .cmp(&b.zip_code)
            .then(a.town.cmp(&b.town))
            .then(a.last_name.cmp(&b.last_name))
    });

    // look for people that live in the same place
    let mut couvert_infos: Vec<pdfgen::CouvertInfo> = Vec::with_capacity(people.len());
    let first_person: &dbparse::ReasonablePerson = &people.get(0).unwrap();
    let mut couvert_info: pdfgen::CouvertInfo = pdfgen::CouvertInfo {
        receivers: Vec::<pdfgen::Receiver>::new(),
        address: get_address(first_person, /*use family:*/ false),
    };
    couvert_info
        .receivers
        .push(into_receiver(first_person, &mapping));
    couvert_infos.push(couvert_info);
    let mut previous_family_address = get_address(first_person, false);

    for person in people.iter().skip(1) {
        let addr_family = get_address(person, true);
        let receiver = into_receiver(person, &mapping);

        if addr_family == previous_family_address && !normalize_but_dont_merge {
            // add to previous couvert another receiver
            couvert_infos.last_mut().unwrap().receivers.push(receiver);
            couvert_infos.last_mut().unwrap().address = addr_family;
        } else {
            let mut couvert_info: pdfgen::CouvertInfo = pdfgen::CouvertInfo {
                receivers: Vec::<pdfgen::Receiver>::new(),
                address: get_address(person, /*family:*/ false),
            };
            couvert_info.receivers.push(receiver);
            couvert_infos.push(couvert_info);

            previous_family_address = addr_family.clone();
        }
    }

    // sort by nickname within household
    for couvert in couvert_infos.iter_mut() {
        couvert
            .receivers
            .sort_by(|ra: &pdfgen::Receiver, rb: &pdfgen::Receiver| ra.nickname.cmp(&rb.nickname));
    }

    return couvert_infos;
}

/// removes newlines within address
/// trims starting and ending whitespace
/// replaces "str."  with "strasse" and replaces " str." with "Strasse"
/// ```
/// # use combine::normalize_address;
/// let addr : String = String::from(" add\nressstr.  ");
/// let normalized : String = normalize_address(&addr);
/// assert_eq!(normalized, String::from("addressstrasse"))
/// ```
pub fn normalize_address(address: &String) -> String {
    let trimmed = address.trim().replace("\n", "").replace("\r", "");
    let rgx_str1 = regex::Regex::new(r"str\.").unwrap();
    let rgx_str2 = regex::Regex::new(r"\sstr.").unwrap();
    let replaced1 = rgx_str1.replace_all(&trimmed, "strasse");
    let replaced2 = rgx_str2.replace_all(&replaced1, "Strasse");
    return String::from(replaced2);
}

fn warn_if_address_incomplete(
    person: &dbparse::ReasonablePerson,
    user_interface: &dyn ui::UserInteractor,
) -> bool {
    let issue: bool = person.first_name.is_empty()
        || person.last_name.is_empty()
        || person.address.is_empty()
        || person.zip_code.is_empty()
        || person.town.is_empty();

    if issue {
        user_interface.report_bad_address(person);
    }

    return issue;
}

/// replaces Pfäffikon, Pfaeffikon, etc with "Pfäffikon ZH"
/// ## testcases
/// ```
/// let town = combine::normalize_town(&String::from("Pfaeffikon "));
/// assert_eq!(String::from("Pfäffikon ZH"), town);
/// ```
///
/// ```
/// # use combine::normalize_town;
/// # let t = &String::from(" pfa\neffikon ZH ");
/// assert_eq!(String::from("Pfäffikon ZH"), normalize_town(t));
/// ```
///
pub fn normalize_town(town: &String) -> String {
    let trimmed = town.trim().replace("\n", "").replace("\r", "");
    let rgx = regex::Regex::new(r"(?i)Pf(ae|ä)ffikon(\s?ZH)?").unwrap();
    return String::from(
        rgx.replace_all(&trimmed, /*replace with:*/ "Pfäffikon ZH")
            .trim(),
    );
}

fn get_address(
    person: &dbparse::ReasonablePerson,
    use_familie_instead_of_first_name: bool,
) -> Vec<String> {
    let first_or_family = if use_familie_instead_of_first_name {
        String::from("Familie")
    } else {
        person.first_name.clone()
    };
    vec![
        format!("{} {}", first_or_family, person.last_name),
        person.address.clone(),
        format!("{} {}", person.zip_code, person.town),
    ]
}

fn into_receiver(
    person: &dbparse::ReasonablePerson,
    group_mapping: &dbparse::mapping::GroupMapping,
) -> pdfgen::Receiver {
    let pdfgen_roles = person
        .roles
        .iter()
        .map(|x| roletranslation::role_to_role(x));
    let mut best_pdfgen_role: pdfgen::Role = pdfgen_roles
        .max_by_key(|x| x.priority())
        .unwrap_or(pdfgen::Role::Nothing);

    let best_group_perhaps: Option<&dbparse::ReasonableGroup> =
        person.groups.iter().max_by_key(|x| x.priority());
    let display_name = match best_group_perhaps {
        Some(group) => group_mapping
            .get_display_name(&group.inner_group.id)
            .expect("Group id does not exist. Something is messed up."),
        None => String::from(""),
    };

    // if nickname is empty, use first name
    let name = match person.nickname.trim().is_empty() {
        true => person.first_name.clone(),
        false => person.nickname.clone(),
    };

    // if the person is of group Trägerkreis, make them a Trägerkreis Role
    if let Some(_index) = display_name.find("Trägerkreis") {
        best_pdfgen_role = pdfgen::Role::Traegerkreis;
    }

    // replace Role::Nothing with the name of the person
    if let pdfgen::Role::Nothing = best_pdfgen_role {
        best_pdfgen_role = pdfgen::Role::Custom(name.clone());
    }

    pdfgen::Receiver {
        nickname: name,
        group: display_name,
        role: best_pdfgen_role,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Priority(i32);
trait Prioritized {
    fn priority(&self) -> Priority;
}
impl Prioritized for pdfgen::Role {
    /// Higher priority is assigned to pdfgen::Role enum variants that should be preferredly
    /// printed on the couverts
    fn priority(&self) -> Priority {
        use pdfgen::Role;
        match self {
            // Logic: just in case somebody is both Leiter, Ehemalige and Trägerkreis
            Role::Traegerkreis => Priority(50),
            Role::Ehemalige => Priority(45),
            Role::Leiter => Priority(40),
            Role::Teilnehmer => Priority(30),
            Role::Nothing => Priority(0),
            _ => Priority(-100), // we don't care whether it's a coach or a Kassier or a Matchef
        }
    }
}

impl Prioritized for dbparse::ReasonableGroup {
    /// Higher priority for more specific groups
    /// I try to use distinct priorities, so that the same dataset will always produce the same
    /// output. That does not hold for non-matches `_`.
    fn priority(&self) -> Priority {
        match self.inner_group.group_type.as_str() {
            "Dachverband" => Priority(10),
            "Mitgliederorganisation" => Priority(30),
            "Sektion" => Priority(40),
            "Verein" => Priority(45),
            "Jungschar" => Priority(49),
            "Gruppe" => Priority(50),     // M oder F
            "Ortsgruppe" => Priority(60), // Pfä-Feh-Hi-Rus
            "Stufe" => Priority(70),
            "Mitglieder" => Priority(71),
            //--- end useless stuff ---
            "Ten-Sing" => Priority(80),
            "Gremium" => Priority(81), // e.g. "C-Gruppe", "Cevi Plus Team"
            "Externe" => match self.inner_group.name.as_str() {
                "Trägerkreis Mitglieder" => Priority(89),
                "J+S-Coaches" => Priority(88),
                "Leiter ehemalig" => Priority(87),
                "Ehemalige" => Priority(87),
                "Gebetsbrunch" => Priority(86),
                "C-Newsletter" => Priority(85),
                "Freie Mitarbeiter" => Priority(84),
                "Z_Import Optigem" => Priority(1),
                "Admin GS 2019" => Priority(3),
                "EXT: Y-Card Aktiv und Gültig" => Priority(2),
                _ => Priority(83),
            },
            "Vorstand" => Priority(90), // if somebody is in a group and in vorstand, we want the group
            "Untergruppe" => Priority(100), // We want always this. It's e.g. Holon.
            "Fröschli" => Priority(100), // Whyever this exists
            // if something is not in this list, we don't want it in almost all cases
            _ => Priority(0),
        }
    }
}

/// this exists solely to avoid cyclic dependencies from ui to dbparse and back
struct DbparseRedirector<'a> {
    user_interface: Option<&'a dyn ui::UserInteractor>,
}
impl<'a> dbparse::DbparseInteractor for DbparseRedirector<'a> {
    fn on_download_finished(&self) {
        match self.user_interface {
            None => (),
            Some(addr) => addr.on_download_finished(),
        }
    }

    fn error_missing_config_file(&self, filename: String) {
        match self.user_interface {
            None => (),
            Some(ui) => ui.error_missing_config_file(filename),
        }
    }
}

/// remove ALL settings if remove_config is true, otherwise only remove all files that are not
/// required for the program to "successfully" run. That is, the program might generate crappy
/// couverts, but it should still generate couverts.
///
/// if `test_run` is set, does not remove anything, only prints
pub fn clean(
    remove_config: bool,
    test_run: bool,
    uiopt: Option<&dyn ui::UserInteractor>,
) -> std::io::Result<()> {
    // in order to delete the files, we create the files first if they don't exist. Because that's
    // the easiest, if hackiest, way to avoid crashing because we try to remove a file that isn't
    // there. I know that somebody could delete the file in the meantime... well if they do that,
    // they deserve the crash!
    use std::fs::OpenOptions;

    // delete injection file
    if !test_run {
        let _r: Option<()> = uiopt.and_then(|ui| {
            ui.inform_user(&*format!(
                "Removing Injection File: {}",
                crate::injection::INJECTION_YAML_FILE_PATH
            ));
            None
        });
        // see note at the start of this function
        {
            let _file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(crate::injection::INJECTION_YAML_FILE_PATH);
        }
        std::fs::remove_file(crate::injection::INJECTION_YAML_FILE_PATH)?;
        // create empty injection file template
        crate::injection::create_injection_yaml_file_template()?;
    } else {
        let _r: Option<()> = uiopt.and_then(|ui| {
            ui.inform_user(&*format!(
                "Would remove Injection File: {}",
                crate::injection::INJECTION_YAML_FILE_PATH
            ));
            None
        });
    }

    // delete mapping yaml file
    if !test_run {
        let _r: Option<()> = uiopt.and_then(|ui| {
            ui.inform_user(&*format!(
                "Removing Mapping File: {}",
                dbparse::MAPPING_YAML_FILE
            ));
            None
        });
        // see note at the start of this function
        {
            let _file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(dbparse::MAPPING_YAML_FILE);
        }
        std::fs::remove_file(dbparse::MAPPING_YAML_FILE)?;
    } else {
        let _r: Option<()> = uiopt.and_then(|ui| {
            ui.inform_user(&*format!(
                "Would remove Mapping File: {}",
                dbparse::MAPPING_YAML_FILE
            ));
            None
        });
    }

    // delete config.yaml file
    if !test_run {
        if remove_config {
            let _r: Option<()> = uiopt.and_then(|ui| {
                ui.inform_user(&*format!(
                    "Removing Config File (necessary to run): {}",
                    dbparse::CONFIG_YAML_FILE
                ));
                None
            });
            // see note at the start of this function
            {
                let _file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(dbparse::CONFIG_YAML_FILE);
            }
            std::fs::remove_file(dbparse::CONFIG_YAML_FILE)?;
        }
    } else {
        if remove_config {
            let _r: Option<()> = uiopt.and_then(|ui| {
                ui.inform_user(&*format!(
                    "Would remove Config File (necessary to run): {}",
                    dbparse::CONFIG_YAML_FILE
                ));
                None
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {}
