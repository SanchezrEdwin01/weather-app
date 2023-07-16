use yew::prelude::*;
use yew::html::{InputData};

pub struct Form {
    city: String,
    temperature: f64,
    description: String,
    humidity: f64,
}

pub enum Msg {
    UpdateCity(String),
    UpdateTemperature(String),
    UpdateDescription(String),
    UpdateHumidity(String),
    SubmitForm,
}

impl Component for Form {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Form {
            city: String::new(),
            temperature: 0.0,
            description: String::new(),
            humidity: 0.0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateCity(city) => self.city = city,
            Msg::UpdateTemperature(temperature) => {
                if let Ok(temperature) = temperature.parse::<f64>() {
                    self.temperature = temperature;
                }
            }
            Msg::UpdateDescription(description) => self.description = description,
            Msg::UpdateHumidity(humidity) => {
                if let Ok(humidity) = humidity.parse::<f64>() {
                    self.humidity = humidity;
                }
            }
            Msg::SubmitForm => {
                // Aquí puedes realizar las acciones necesarias con los datos del formulario
                // Por ejemplo, enviar los datos al backend para crear un nuevo elemento
                // o llamar a una función de controlador correspondiente
                // ...
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            // Vista del formulario
            <form onsubmit={|e: SubmitEvent| {
                    e.prevent_default();
                    Msg::SubmitForm;
                    ()
                }}>
                <div>
                    <label for="city">{"City:"}</label>
                    <input
                        type="text"
                        id="city"
                        value={self.city.clone()}
                        oninput={|e: InputData| Msg::UpdateCity(e.value)}
                        required=true
                    />
                </div>
                <div>
                    <label for="temperature">{"Temperature:"}</label>
                    <input
                        type="number"
                        id="temperature"
                        step="0.01"
                        value={self.temperature.to_string()}
                        oninput={|e: InputData| Msg::UpdateTemperature(e.value)}
                        required=true
                    />
                </div>
                <div>
                    <label for="description">{"Description:"}</label>
                    <input
                        type="text"
                        id="description"
                        value={self.description.clone()}
                        oninput={|e: InputData| Msg::UpdateDescription(e.value)}
                        required=true
                    />
                </div>
                <div>
                    <label for="humidity">{"Humidity:"}</label>
                    <input
                        type="number"
                        id="humidity"
                        step="0.01"
                        value={self.humidity.to_string()}
                        oninput={|e: InputData| Msg::UpdateHumidity(e.value)}
                        required=true
                    />
                </div>
                <div>
                    <button type="submit">{"Submit"}</button>
                </div>
            </form>
        }
    }
}
