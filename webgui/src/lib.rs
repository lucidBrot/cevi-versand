#![recursion_limit="256"]
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use combine;

pub struct Model {
    email: String,
    password: String,
    auth_token: String,
    buttontext: String,
    debugtext: String,
}

pub enum Msg {
    Click,
    StartDownload,
    StartDownloading,
    DoneDownloading(seed::fetch::ResponseDataResult<String>),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            email: String::new(),
            password: String::new(),
            auth_token: String::new(),
            buttontext: "Auth token holen".to_string(),
            debugtext: "nothing to report".to_string(),
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
                seed::Request::new(url.to_string()).fetch_string_data(Msg::DoneDownloading);
            },
            Msg::DoneDownloading(Ok(data)) => {
                self.debugtext = format!("data received: {}", data);
            },
            Msg::DoneDownloading(Err(fail_reason)) => {
                self.debugtext = format!("no data received: {:?}", fail_reason);
            }
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


