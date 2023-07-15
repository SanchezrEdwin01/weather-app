use yew::prelude::*;
use crate::models::WeatherItem;

pub struct List {
    items: Vec<WeatherItem>,
}

pub enum Msg {}

impl Component for List {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        // Inicializar el estado del componente con una lista vacía de elementos
        List { items: Vec::new() }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        // No se realizan actualizaciones en este componente
        false
    }

    fn view(&self) -> Html {
        html! {
            // Vista de la página de lista de elementos
            <div>
                <h1>{"Items List"}</h1>
                <ul>
                    { for self.items.iter().map(|item| self.render_item(item)) }
                </ul>
            </div>
        }
    }
}

impl List {
    fn render_item(&self, item: &WeatherItem) -> Html {
        html! {
            // Vista de cada elemento de la lista
            <li>
                <div>{ format!("City: {}", item.city) }</div>
                <div>{ format!("Temperature: {}", item.temperature) }</div>
                <div>{ format!("Description: {}", item.description) }</div>
                <div>{ format!("Humidity: {}", item.humidity) }</div>
                // Otros campos del elemento
            </li>
        }
    }
}
