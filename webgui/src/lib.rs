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
    StartDownload
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
                self.buttontext = "tested_internet".to_string();
                use internetleek::InternetLeek;
                let a = internetleek::get_internet_leek().GET_body("https://docs.rs/seed/0.2.8/seed/fetch/struct.Request.html");
                if let Ok(s) = a {
                    self.debugtext = s;
                }
            },
            Msg::StartDownload => {combine::main();} // TODO: use correct function call
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
                <button onclick=|_| Msg::StartDownload>{ "Loslegen!" }</button>
                <hr/>
                <p>{ &self.debugtext }</p>
            </div>
        }
    }
}


