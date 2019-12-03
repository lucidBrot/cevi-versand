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
pub struct WebInternetLeek {}
#[cfg(target_arch = "wasm32")]
impl WebInternetLeek {
    fn new() -> Self {
        WebInternetLeek{}
    }
}

#[cfg(target_arch = "wasm32")]
impl InternetLeek for WebInternetLeek {
    fn GET_body (&self, url: &str) -> Result<String, Box<dyn std::error::Error>>  {
        let request = seed::fetch::Request::new(url)
            .method(seed::fetch::Method::Get);
        use futures::future;
        use std::future::Future;
        let future_result = request.fetch_string_data(|_| ());
        let result = futures::executor::block_on<futures::future::Future>(future_result.into());
        // TODO: find a way that will not block the event loop
        return result;
    }
}
