use actix_web::{web, Responder, HttpResponse};
use crate::AppState;

pub async fn delete_item(data: web::Data<AppState>, item_id: web::Path<u32>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    if let Some(pos) = items.iter().position(|i| i.id == *item_id) {
        items.remove(pos);
        HttpResponse::Ok().body("Item deleted")
    } else {
        HttpResponse::NotFound().body("Item not found")
    }
}
