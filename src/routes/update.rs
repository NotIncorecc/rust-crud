use actix_web::{web, Responder, HttpResponse};
use crate::AppState;
use crate::routes::models::Item;

pub async fn update_item(data: web::Data<AppState>, item: web::Json<Item>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    if let Some(existing) = items.iter_mut().find(|i| i.id == item.id) {
        existing.name = item.name.clone();
        HttpResponse::Ok().body("Item updated")
    } else {
        HttpResponse::NotFound().body("Item not found")
    }
}
