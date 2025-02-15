use actix_web::{web, Responder, HttpResponse};
use crate::AppState;
use crate::routes::models::Item;

pub async fn create_item(data: web::Data<AppState>, item: web::Json<Item>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    items.push(item.into_inner());
    HttpResponse::Created().body("Item created")
}