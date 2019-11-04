use super::StringHashMap;
use std::collections::{HashMap, HashSet};
use super::ReasonableGroup;

//pub fn create_yaml_from_map();
//pub fn create_map_from_yaml();
//pub fn store_map_in_map(priority_map, old_map)

pub fn create_yaml_from_set(set: &HashSet<ReasonableGroup>) {

}

type GroupID = String;
struct GroupNames {
    original_name : String,
    display_name : Option<String>,
}
struct GroupMapping {
    map: HashMap<GroupID, GroupNames>,
}
impl GroupMapping {
    fn get_display_name(&self, group_id: &GroupID) -> Option<String> {
        let entry : Option<&GroupNames> = self.map.get(group_id);
        match entry {
            None => None,
            Some(group_names) => group_names.display_name.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StringHashMap;
    use super::{GroupMapping, GroupMappingStruct};

    #[test]
    fn test_map_one_element(){
        let mut m = StringHashMap::new();
        let s1 = String::from("hello");
        let s2 = String::from("there");
        m.insertt(s1, s2);
        assert_eq!("there", m.get("hello").expect("Should be something"));
    }
}
