#![recursion_limit="256"]
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::services::fetch::{FetchService, Response, Request};
use combine;

pub struct Model {
    email: String,
    password: String,
    auth_token: String,
    buttontext: String,
    debugtext: String,
    fetch_service: FetchService,
    component_link: ComponentLink<Self>,
}

pub enum Msg {
    Click,
    StartDownload,
    StartDownloading,
    DoneDownloading(Result<String, failure::Error>),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            email: String::new(),
            password: String::new(),
            auth_token: String::new(),
            buttontext: "Auth token holen".to_string(),
            debugtext: "nothing to report".to_string(),
            fetch_service: FetchService::new(),
            component_link: link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.buttontext = "useless button".to_string();
            },
            Msg::StartDownload => {combine::main();}, // TODO: use correct function call
            Msg::StartDownloading => {
                let url : &str = "https://seed-rs.org/guide/http-requests-and-state";
                self.download_people_data(url);
            },
            Msg::DoneDownloading(data) => {
                self.debugtext = format!("data received: {:?}", data);
            },
        }
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <input type="email" placeholder="ich@cevi.ch">{ &self.email }</input>
                <br/>
                <input type="password" placeholder="Passwort">{ &self.password }</input>
                <br/>
                <button onclick=|_| Msg::Click>{ &self.buttontext }</button>
                <br/>
                <input type="text">{ &self.auth_token }</input>
                <br/>
                <button onclick=|_| Msg::StartDownloading>{ "Loslegen!" }</button>
                <hr/>
                <p>{ &self.debugtext }</p>
            </div>
        }
    }
}

impl Model {
    fn download_people_data(&mut self, uri: &str) {
        let request = Request::get(uri).body(()).expect("Failed to build request");
        let callback = self.component_link.send_back(move |response: Response<Result<String, _>>| {
            if response.status().is_success() {
                return Msg::DoneDownloading(response.into_body());
            } else {
                return Msg::DoneDownloading(response.into_body());
            }
        });
        //self.fetch_service.fetch(request, callback);

    }
}
