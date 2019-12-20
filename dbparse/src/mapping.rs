use super::ReasonableGroup;
use super::{Verbosity, VERBOSITY};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

const ABTEILUNG : &str = " Pfäffikon-Fehraltorf-Hittnau-Russikon";
const VERBOSE : bool = false;

/// turns a given yaml String into a GroupMapping
pub fn create_map_from_yaml(yaml_str : &str) -> Result<GroupMapping, serde_yaml::Error> {
    let map_opt : Result<GroupMapping, serde_yaml::Error> = serde_yaml::from_str(yaml_str);
    return map_opt;
}

/// merges the two maps. When both maps contain the same key, the entry from `priority_map` is
/// taken.
pub fn store_map_in_map(priority_map : &GroupMapping, old_map : &GroupMapping) -> GroupMapping {
    let mut new_map : GroupMapping = (*priority_map).clone();
    for (key, value) in old_map.map.iter() {
        if let None = priority_map.map.get(key) {
            new_map.map.insert(key.to_owned(), value.clone());
        }
    }
    return new_map;
}

pub fn create_yaml_from_map(map : &GroupMapping) -> Option<String> {
    let my_yaml : Result<String, _> = serde_yaml::to_string(&map);
    match my_yaml {
        Ok(content_string) => {
            if VERBOSITY.value() >= Verbosity::ABit.value() {
                println!("yaml: \n{}", content_string);
            }
            return Some(content_string);
        },
        Err(e) => {
            println!("yaml serializing error: \n{}", e);
            return None;
        },
    };
}

type GroupID = String;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupNames {
    original_name : String,
    display_name : Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupMapping {
    pub map : HashMap<GroupID, GroupNames>,
}
impl GroupMapping {
    pub fn get_display_name(&self, group_id : &GroupID) -> Option<String> {
        let entry : Option<&GroupNames> = self.map.get(group_id);
        match entry {
            None => None,
            Some(group_names) => group_names.display_name.clone(),
        }
    }

    pub fn new() -> Self {
        GroupMapping {
            map : HashMap::new(),
        }
    }

    pub fn from_set(set : &HashSet<ReasonableGroup>) -> Self {
        let mut group_mapping = GroupMapping::new();
        for group in set.iter() {
            group_mapping.map.insert(
                group.inner_group.id.clone(),
                GroupNames {
                    original_name : group.inner_group.name.clone(),
                    display_name : Some(Self::autocorrect_group_name(&*group.inner_group.name)),
                },
            );
        }
        return group_mapping;
    }

    fn autocorrect_group_name(name : &str) -> String {
        // automapping specific to Abteilung PFA, and
        // remove (F)  and (M)
        const F : &str = " (F)";
        const M : &str = " (M)";
        // Abteilung could also be found generically by searching for a group with group_type
        // Ortsgruppe and taking its name
        let mut result = String::from(
            name.clone()
                .trim_end_matches(F)
                .trim_end_matches(M)
                .trim_end_matches(ABTEILUNG)
                .trim(),
        );

        // automatically truncate "Trägerkreis Mitglieder" into "Trägerkreis"
        if result == "Trägerkreis Mitglieder" {
            result = "Trägerkreis".to_string();
        }

        if VERBOSE {
            println!("autocorrect: {}  =>  {}", name, result);
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::super::StringHashMap;

    #[test]
    fn test_map_one_element() {
        let mut m = StringHashMap::new();
        let s1 = String::from("hello");
        let s2 = String::from("there");
        m.insertt(s1, s2);
        assert_eq!("there", m.get("hello").expect("Should be something"));
    }

    #[test]
    fn test_autocorrect_pfa() {
        let inp = "Fröschli Pfäffikon-Fehraltorf-Hittnau-Russikon";
        let out = super::GroupMapping::autocorrect_group_name(inp);
        assert_eq!(out, "Fröschli");
    }

    #[test]
    fn test_autocorrect_m() {
        let inp = "Holon (M)";
        let out = super::GroupMapping::autocorrect_group_name(inp);
        assert_eq!(out, "Holon");
    }
}
