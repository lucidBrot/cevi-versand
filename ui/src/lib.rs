#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/// related to [issue 5](https://github.com/lucidBrot/cevi-versand/issues/5)  
///
/// An interface that allows internal functions to inform the user about something
pub trait UserInteractor {
    fn on_download_finished(&self){}
    fn on_parsing_finished(&self){}
    fn report_bad_addresses(&self, broken_people: Vec<dbparse::ReasonablePerson>){}
    fn on_pdf_generation_finished(&self){}
}

/// Simplistic default user interface
pub struct CliUi {}

impl UserInteractor for CliUi {
    fn on_download_finished(&self){
        println!("UI: Download Finished.");
    }

    fn on_parsing_finished(&self){
        println!("UI: Parsing Finished.");
    }

    fn report_bad_addresses(&self, broken_people: Vec<dbparse::ReasonablePerson>) {
        println!("UI: Found {} people with broken addresses.
                 \n{:?}", broken_people.len(), broken_people);
    }

    fn on_pdf_generation_finished(&self) {
        println!("UI: Finished generating pdf");
    }

}
