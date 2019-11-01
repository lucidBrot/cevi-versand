use super::StringHashMap;

//pub fn create_yaml_from_map();
//pub fn create_map_from_yaml();
//pub fn store_map_in_map(priority_map, old_map)

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
