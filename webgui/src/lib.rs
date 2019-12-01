use yew::{html, Component, ComponentLink, Html, ShouldRender};
use combine;

pub struct Model {
    email: String,
    password: String,
    auth_token: String,
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
            auth_token: String::new()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.email = "herbert".to_string();
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
                <button onclick=|_| Msg::Click>{ "Auth Token holen" }</button>
                <br/>
                <input type="text">{ &self.auth_token }</input>
                <br/>
                <button onclick=|_| Msg::StartDownload>{ "Loslegen!" }</button>
            </div>
        }
    }
}


