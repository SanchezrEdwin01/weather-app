use crate::services::item_service;
use crate::database::DbPool;
use crate::models::WeatherItem;
use yew::prelude::*;
use yew::functional::{use_state, use_callback};

pub struct ListController {
    pool: DbPool,
    items: Vec<WeatherItem>,
    state: i32,
    callback: Callback<Msg>,
}

pub enum Msg {
    FetchItems,
    ItemsFetched(Result<Vec<WeatherItem>, String>),
}

impl Component for ListController {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties) -> Self {
        // Inicializar el pool de la base de datos y otras configuraciones necesarias
        let pool = crate::database::establish_connection();

        // Obtener los elementos al crear el controlador
        let (state, set_state) = use_state(|| 0);
        let callback = use_callback(|msg: Msg| set_state(|_| 0) && msg);

        callback.emit(Msg::FetchItems);

        ListController {
            pool,
            items: Vec::new(),
            state,
            callback,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchItems => {
                // Obtener los elementos de la base de datos
                let callback = self.callback.reform(|result: Result<Vec<WeatherItem>, String>| Msg::ItemsFetched(result));
                let task = crate::service::item_service::get_all_weather_items(&self.pool)
                    .map_err(|err| err.to_string())
                    .and_then(callback);
                yew::services::spawn(task);
                true // Return true to re-render the component
            }
            Msg::ItemsFetched(Ok(items)) => {
                // Actualizar los elementos obtenidos
                self.items = items;
                true // Return true to re-render the component
            }
            Msg::ItemsFetched(Err(error)) => {
                // Manejar el error al obtener los elementos
                log::error!("Error fetching items: {}", error);
                // Aquí puedes mostrar un mensaje de error al usuario si lo deseas
                false // Return false to avoid re-rendering the component
            }
        }
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
