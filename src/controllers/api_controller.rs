use crate::services::item_service;
use crate::database::DbPool;
use crate::models::WeatherItem;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

pub async fn get_all_items(pool: web::Data<DbPool>) -> impl Responder {
    match item_service::get_all_weather_items(&pool) {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_item(item_id: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    match item_service::get_weather_item(&pool, *item_id) {
        Ok(Some(item)) => HttpResponse::Ok().json(item),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn create_item(item: web::Json<WeatherItem>, pool: web::Data<DbPool>) -> impl Responder {
    match item_service::create_weather_item(&pool, &item) {
        Ok(new_item) => HttpResponse::Created().json(new_item),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_item(
    item_id: web::Path<i32>,
    updated_item: web::Json<WeatherItem>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    match item_service::update_weather_item(&pool, *item_id, &updated_item) {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_item(item_id: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    match item_service::delete_weather_item(&pool, *item_id) {
        Ok(deleted_rows) => {
            if deleted_rows > 0 {
                HttpResponse::NoContent().finish()
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
