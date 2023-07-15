use crate::services::item_service;
use crate::database::DbPool;
use crate::models::WeatherItem;
use yew::prelude::*;
use yew_router::agent::RouteRequest;
use yew_router::prelude::*;

pub struct FormController {
    link: ComponentLink<Self>,
    pool: DbPool,
    router: RouteAgentDispatcher<()>,
    item: WeatherItem,
}

pub enum Msg {
    UpdateField(String, String),
    SaveItem,
}

impl Component for FormController {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Inicializar el pool de la base de datos y otras configuraciones necesarias
        let pool = crate::database::establish_connection();
        let router = RouteAgentDispatcher::new();

        FormController {
            link,
            pool,
            router,
            item: WeatherItem {
                id: 0,
                city: String::new(),
                temperature: 0.0,
                description: String::new(),
                humidity: 0.0,
                wind: None,
                visibility: None,
                atmospheric_pressure: None,
                sunrise: None,
                sunset: None,
                date_time: chrono::Utc::now(),
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateField(field, value) => {
                // Actualizar el campo correspondiente en el item
                match field.as_str() {
                    "city" => self.item.city = value,
                    "temperature" => {
                        if let Ok(temperature) = value.parse::<f64>() {
                            self.item.temperature = temperature;
                        }
                    }
                    "description" => self.item.description = value,
                    "humidity" => {
                        if let Ok(humidity) = value.parse::<f64>() {
                            self.item.humidity = humidity;
                        }
                    }
                    // Actualizar otros campos si es necesario
                    _ => {}
                }
            }
            Msg::SaveItem => {
                // Guardar el item en la base de datos
                if let Ok(new_item) = item_service::create_weather_item(&self.pool, &self.item) {
                    // Navegar a la lista de elementos después de guardar el item
                    self.router.send(RouteRequest::ChangeRoute(Route::from("/list")));
                }
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            // Vista del formulario
            <form>
                <div>
                    <label>{"City:"}</label>
                    <input
                        type="text"
                        value=self.item.city.clone()
                        oninput=self.link.callback(|e: InputData| Msg::UpdateField("city".to_owned(), e.value))
                    />
                </div>
                <div>
                    <label>{"Temperature:"}</label>
                    <input
                        type="number"
                        step="0.01"
                        value=self.item.temperature.to_string()
                        oninput=self.link.callback(|e: InputData| Msg::UpdateField("temperature".to_owned(), e.value))
                    />
                </div>
                <div>
                    <label>{"Description:"}</label>
                    <input
                        type="text"
                        value=self.item.description.clone()
                        oninput=self.link.callback(|e: InputData| Msg::UpdateField("description".to_owned(), e.value))
                    />
                </div>
                <div>
                    <label>{"Humidity:"}</label>
                    <input
                        type="number"
                        step="0.01"
                        value=self.item.humidity.to_string()
                        oninput=self.link.callback(|e: InputData| Msg::UpdateField("humidity".to_owned(), e.value))
                    />
                </div>
                // Otros campos del formulario
                <div>
                    <button onclick=self.link.callback(|_| Msg::SaveItem)>{"Save"}</button>
                </div>
            </form>
        }
    }
}
