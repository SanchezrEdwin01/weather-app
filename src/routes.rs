use actix_web::web;
use crate::controllers::{
    home_controller,
    list_controller,
    form_controller,
    api_controller,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .route("", web::get().to(home_controller::index))
            .route("list", web::get().to(list_controller::index))
            .route("list/{id}", web::get().to(list_controller::show))
            .route("list/{id}", web::put().to(list_controller::update))
            .route("list/{id}", web::delete().to(list_controller::delete))
            .route("form", web::get().to(form_controller::index))
            .route("form", web::post().to(form_controller::create))
            .service(
                web::scope("/api")
                    .route("items", web::get().to(api_controller::get_items))
                    .route("items/{id}", web::get().to(api_controller::get_item))
                    .route("items", web::post().to(api_controller::create_item))
                    .route("items/{id}", web::put().to(api_controller::update_item))
                    .route("items/{id}", web::delete().to(api_controller::delete_item))
            )
    );
}
