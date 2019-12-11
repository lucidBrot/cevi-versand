/// related to [issue 5](https://github.com/lucidBrot/cevi-versand/issues/5)  
///
/// An interface that allows internal functions to inform the user about something
pub trait UserInteractor {
    fn on_download_finished(&self){}
    fn on_parsing_finished(&self){}
    fn report_bad_address(&self, _broken_person: &dbparse::ReasonablePerson){}
    fn on_pdf_generation_finished(&self){}
    fn error_missing_config_file(&self, _filename: String) {}
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

    fn report_bad_address(&self, broken_person: &dbparse::ReasonablePerson) {
        println!("UI: Broken Address Found:
                 {:?}", broken_person);
    }

    fn on_pdf_generation_finished(&self) {
        println!("UI: Finished generating pdf");
    }

}
