use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Model {
    email: Option<String>,
    email_placeholder: String,
    password: String,
    auth_token: String,
}

pub enum Msg {
    Click,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            email: None::<String>,
            email_placeholder: "ich@cevi.ch".to_string(),
            password: String::new(),
            auth_token: String::new()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.email = Some("herbert".to_string());
            }
        }
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <input type="email">{ &self.email_text() }</input>
                <br/>
                <input type="password">{ &self.password }</input>
                <br/>
                <input type="text">{ &self.auth_token }</input>
                <button onclick=|_| Msg::Click>{ &self.email_text() }</button>
            </div>
        }
    }
}

impl Model {
    /// get display text for email textbox
    fn email_text(&self) -> &str {
        match &self.email {
            None => self.email_placeholder.as_ref(),
            Some(x) => x.as_ref(),
        }
    }
}
