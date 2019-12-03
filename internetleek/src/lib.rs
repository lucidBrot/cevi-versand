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

#[cfg(target_arch = "wasm32")]
pub struct WebInternetLeek { data: Option<String>}
#[cfg(target_arch = "wasm32")]
impl WebInternetLeek {
    fn new() -> Self {
        WebInternetLeek{ data: None, }
    }
}

#[cfg(target_arch = "wasm32")]
impl InternetLeek for WebInternetLeek {
    fn GET_body (&self, url: &str) -> Result<String, Box<dyn std::error::Error>>  {
        match &self.data {
            Some(a) => Ok(a.to_string()),
            None => Err(Box::new(GenericAsFuckError::new())),
        }
    }
}

#[derive(Debug)]
pub struct GenericAsFuckError {}
impl std::error::Error for GenericAsFuckError {
    fn description(&self) -> &str {
        "Anything might have happened"
    }
}
impl std::fmt::Display for GenericAsFuckError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", "Anything might have happened".to_string())
    }
}
impl GenericAsFuckError {
    fn new() -> Self {
        GenericAsFuckError {}
    }
}
