#[derive(Debug, Clone)]
pub struct Text {
    singular_text : String,
    plural_text : String,
}

pub trait Pluralizable {
    fn new(singular: &str, plural: &str) -> Self;
    
    /// get the singular version of the given Text
    fn singular(&self) -> &str;

    /// get the pluralized version of the given Text
    fn plural(&self) -> &str;

    /// get either the singular or plural version
    fn for_num(&self, num: usize) -> String{
        match num {
            1 => String::from(self.singular()),
            _ => String::from(self.plural())
        }
    }
}

impl Pluralizable for Text {
    fn singular(&self) -> &str {&self.singular_text}
    fn plural(&self) -> &str {&self.plural_text}
    fn new(singular: &str, plural : &str) -> Self {Text{
        singular_text: String::from(singular), plural_text: String::from(plural)
    }}
}
