use yew::prelude::*;
use yew_router::agent::RouteRequest;
use yew_router::prelude::*;

pub struct HomeController {
    link: ComponentLink<Self>,
    router: RouteAgentDispatcher<()>,
}

pub enum Msg {
    GoToAllItems,
}

impl Component for HomeController {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        HomeController {
            link,
            router: RouteAgentDispatcher::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GoToAllItems => {
                // Navegar a la página de lista de elementos
                self.router.send(RouteRequest::ChangeRoute(Route::from("/list")));
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
