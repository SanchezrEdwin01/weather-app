use yew::prelude::*;
use yew_router::prelude::*;

pub struct Home {
    link: ComponentLink<Self>,
}

pub enum Msg {
    GoToAllItems,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Home { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GoToAllItems => {
                // Navegar a la página de lista de elementos
                let route = Route::from("/list");
                self.link.send_message(RouteAgentBridge::new(route));
            }
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            // Vista de la página de inicio
            <div>
                <h1>{"Welcome to the Home Page!"}</h1>
                <button onclick=self.link.callback(|_| Msg::GoToAllItems)>{"View All Items"}</button>
            </div>
        }
    }
}
