pub trait InternetLeek {
    #[allow(non_snake_case)]
    fn GET_body (url: &str) -> Result<String, Box<dyn std::error::Error>>;
}

pub struct CliInternetLeek {

}

impl InternetLeek for CliInternetLeek {
    fn GET_body (url: &str) -> Result<String, Box<dyn std::error::Error>>  {
        Ok(chttp::get(url)?.into_body().text()?)
    }
}
