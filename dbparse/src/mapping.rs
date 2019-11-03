use super::StringHashMap;
use std::collections::HashSet;
use super::ReasonableGroup;

//pub fn create_yaml_from_map();
//pub fn create_map_from_yaml();
//pub fn store_map_in_map(priority_map, old_map)

pub fn create_yaml_from_set(set: &HashSet<ReasonableGroup>) {

}

// Make group have a display() function
trait DisplayableAs<T> {
    /// returns a displayable value. E.g. a String, corrected so that it would make sense to show
    /// to a enduser
    fn display (&self) -> T; 
}
trait GroupMapping {
    fn get_display_name (&self, group_name: &str) -> Option<String>;
}
struct DisplayableGroup<'a> {
    group: ReasonableGroup,
    group_mapping: &'a dyn GroupMapping,
}
impl<'a> DisplayableGroup<'a> {
    fn from (r: ReasonableGroup, m: &'a dyn GroupMapping) -> Self {
        DisplayableGroup { group: r, group_mapping: m, }
    }
}
impl DisplayableAs<String> for DisplayableGroup<'_> {
    fn display (&self) -> String {
        self.group_mapping.get_display_name(&self.group.inner_group.name).unwrap_or(self.group.inner_group.name.clone())
    }
}

struct GroupMappingStruct {}
impl GroupMapping for GroupMappingStruct {
    fn get_display_name(&self, group_name: &str) -> Option<String> {
        None
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

    #[test]
    fn test_group_mapping(){
        let gms = GroupMappingStruct{};
        let x = gms.get_display_name(&"holon");
        assert!(x.is_some(), "should be some group name");
    }
}
