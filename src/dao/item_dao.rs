use crate::database::DbPool;
use crate::models::WeatherItem;
use diesel::prelude::*;
use diesel::result::Error;

pub fn create_item(pool: &DbPool, item: &WeatherItem) -> Result<WeatherItem, Error> {
    use crate::schema::weather::dsl::*;

    let conn = pool.get().expect("Failed to obtain database connection");

    let new_item = diesel::insert_into(weather)
        .values(item)
        .get_result(&mut conn)?;
    Ok(new_item)
}

pub fn get_all_items(pool: &DbPool) -> Result<Vec<WeatherItem>, Error> {
    use crate::schema::weather::dsl::*;

    let conn = pool.get().expect("Failed to obtain database connection");

    let items = weather.load::<WeatherItem>(&conn)?;

    Ok(items)
}

pub fn get_item(pool: &DbPool, item_id: i32) -> Result<Option<WeatherItem>, Error> {
    use crate::schema::weather::dsl::*;

    let conn = pool.get().expect("Failed to obtain database connection");

    let item = weather.find(item_id).get_result::<WeatherItem>(&conn).optional()?;

    Ok(item)
}

pub fn update_item(pool: &DbPool, item_id: i32, updated_item: &WeatherItem) -> Result<WeatherItem, Error> {
    use crate::schema::weather::dsl::*;

    let conn = pool.get().expect("Failed to obtain database connection");

    let item = diesel::update(weather.find(item_id))
        .set(updated_item)
        .get_result(&mut conn)?;

    Ok(item)
}

pub fn delete_item(pool: &DbPool, item_id: i32) -> Result<usize, Error> {
    use crate::schema::weather::dsl::*;

    let conn = pool.get().expect("Failed to obtain database connection");

    let deleted_rows = diesel::delete(weather.find(item_id)).execute(&mut conn)?;

    Ok(deleted_rows)
}
