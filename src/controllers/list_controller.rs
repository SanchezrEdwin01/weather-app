use crate::services::item_service;
use crate::database::DbPool;
use crate::models::WeatherItem;
use yew::prelude::*;

pub struct ListController {
    link: ComponentLink<Self>,
    pool: DbPool,
    items: Vec<WeatherItem>,
}

pub enum Msg {
    FetchItems,
    ItemsFetched(Result<Vec<WeatherItem>, String>),
}

impl Component for ListController {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Inicializar el pool de la base de datos y otras configuraciones necesarias
        let pool = crate::database::establish_connection();

        // Obtener los elementos al crear el controlador
        link.send_message(Msg::FetchItems);

        ListController {
            link,
            pool,
            items: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchItems => {
                // Obtener los elementos de la base de datos
                let callback = self.link.callback(|result: Result<Vec<WeatherItem>, String>| Msg::ItemsFetched(result));
                let task = crate::service::item_service::get_all_weather_items(&self.pool)
                    .map_err(|err| err.to_string())
                    .and_then(callback);
                yew::services::spawn(task);
            }
            Msg::ItemsFetched(Ok(items)) => {
                // Actualizar los elementos obtenidos
                self.items = items;
            }
            Msg::ItemsFetched(Err(error)) => {
                // Manejar el error al obtener los elementos
                log::error!("Error fetching items: {}", error);
                // Aquí puedes mostrar un mensaje de error al usuario si lo deseas
            }
        }
        true
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

impl ListController {
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
