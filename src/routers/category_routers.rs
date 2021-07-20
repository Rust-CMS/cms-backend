use actix_web::{web, Scope};

use crate::controllers::category_controllers::*;

pub struct CategoryRouter;

impl CategoryRouter {
    pub fn new() -> Scope {
        web::scope("/category")
            .route("", web::post().to(create_category))
            .route("/{id}", web::get().to(get_category))
            .route("/{id}", web::put().to(update_category))
            .route("/{id}", web::delete().to(delete_category))
    }
}
