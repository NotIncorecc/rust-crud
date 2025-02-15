use actix_web::{web, Responder};
use crate::AppState;
//use crate::routes::models::Item;

pub async fn get_items(data: web::Data<AppState>) -> impl Responder {
    let items = data.items.lock().unwrap();
    web::Json(items.clone())
}
