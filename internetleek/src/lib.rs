pub trait InternetLeek {
    #[allow(non_snake_case)]
    fn GET_body (url: &str) -> Result<String, Box<dyn std::error::Error>>;
}

#[cfg(not(target_arch = "asm32"))]
pub struct CliInternetLeek {}
#[cfg(not(target_arch = "asm32"))]
impl CliInternetLeek {
    fn new() -> Self {
        CliInternetLeek{}
    }
}


#[cfg(not(target_arch = "asm32"))]
impl InternetLeek for CliInternetLeek {
    fn GET_body (url: &str) -> Result<String, Box<dyn std::error::Error>>  {
        Ok(chttp::get(url)?.into_body().text()?)
    }
}
