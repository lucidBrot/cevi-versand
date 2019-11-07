use super::StringHashMap;
use std::collections::{HashMap, HashSet};
use super::ReasonableGroup;
use serde::{Serialize, Deserialize};

//pub fn create_yaml_from_map();
//pub fn create_map_from_yaml();
//pub fn store_map_in_map(priority_map, old_map)

pub fn create_yaml_from_map(map: &GroupMapping){
    let my_yaml : Result<String, _> = serde_yaml::to_string(&map);
    match my_yaml {
        Ok(content_string) => println!("yaml: \n{}", content_string),
        Err(e) => println!("yaml serializing error: \n{}", e)
    };
}

pub fn create_yaml_from_set(set: &HashSet<ReasonableGroup>) {
    let group_mapping : GroupMapping = GroupMapping::from_set(set);
    create_yaml_from_map(&group_mapping);
}

type GroupID = String;
#[derive(Serialize, Deserialize, Debug)]
pub struct GroupNames {
    original_name : String,
    display_name : Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GroupMapping {
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

    fn new() -> Self {
        GroupMapping {map: HashMap::new()}
    }

    fn from_set(set: &HashSet<ReasonableGroup>) -> Self{
        let mut group_mapping = GroupMapping::new();
        for group in set.iter() {
            group_mapping.map.insert(
                group.inner_group.id.clone(),
                GroupNames {
                    original_name: group.inner_group.name.clone(),
                    display_name: Some(group.inner_group.name.clone()),
                    }
                );  
        }
        return group_mapping;
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
