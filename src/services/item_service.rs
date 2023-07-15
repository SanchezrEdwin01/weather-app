use crate::dao::item_dao::{create_item, delete_item, get_all_items, get_item, update_item};
use crate::database::DbPool;
use crate::models::WeatherItem;
use diesel::result::Error;

pub fn create_weather_item(pool: &DbPool, item: &WeatherItem) -> Result<WeatherItem, Error> {
    // Realizar validaciones adicionales aquí si es necesario antes de llamar al DAO
    create_item(pool, item)
}

pub fn get_all_weather_items(pool: &DbPool) -> Result<Vec<WeatherItem>, Error> {
    get_all_items(pool)
}

pub fn get_weather_item(pool: &DbPool, item_id: i32) -> Result<Option<WeatherItem>, Error> {
    get_item(pool, item_id)
}

pub fn update_weather_item(pool: &DbPool, item_id: i32, updated_item: &WeatherItem) -> Result<WeatherItem, Error> {
    // Realizar validaciones adicionales aquí si es necesario antes de llamar al DAO
    update_item(pool, item_id, updated_item)
}

pub fn delete_weather_item(pool: &DbPool, item_id: i32) -> Result<usize, Error> {
    // Realizar validaciones adicionales aquí si es necesario antes de llamar al DAO
    delete_item(pool, item_id)
}
