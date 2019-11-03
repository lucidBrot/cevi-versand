use super::StringHashMap;
use std::collections::HashSet;
use super::ReasonableGroup;

//pub fn create_yaml_from_map();
//pub fn create_map_from_yaml();
//pub fn store_map_in_map(priority_map, old_map)

pub fn create_yaml_from_set(set: &HashSet<ReasonableGroup>) {

}

trait DisplayableAs<T> {
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

#[cfg(test)]
mod tests {
    use super::StringHashMap;

    #[test]
    fn test_map_one_element(){
        let mut m = StringHashMap::new();
        let s1 = String::from("hello");
        let s2 = String::from("there");
        m.insertt(s1, s2);
        assert_eq!("there", m.get("hello").expect("Should be something"));
    }
}
