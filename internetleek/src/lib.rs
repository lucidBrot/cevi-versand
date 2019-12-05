pub fn get_internet_leek () -> impl InternetLeek {
    #[cfg(not(target_arch = "wasm32"))]
    return CliInternetLeek::new();

    #[cfg(target_arch = "wasm32")]
    return WebInternetLeek::new();
}

pub trait InternetLeek {
    #[allow(non_snake_case)]
    fn GET_body (&self, url: &str) -> Result<String, Box<dyn std::error::Error>>;
}

#[cfg(not(target_arch = "wasm32"))]
pub struct CliInternetLeek {}
#[cfg(not(target_arch = "wasm32"))]
impl CliInternetLeek {
    fn new() -> Self {
        CliInternetLeek{}
    }
}


#[cfg(not(target_arch = "wasm32"))]
impl InternetLeek for CliInternetLeek {
    fn GET_body (&self, url: &str) -> Result<String, Box<dyn std::error::Error>>  {
        Ok(chttp::get(url)?.into_body().text()?)
    }
}

