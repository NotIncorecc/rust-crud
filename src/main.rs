mod routes;
use actix_web::{web, App, HttpServer};
use std::sync::{Arc, Mutex};
use routes::{create::create_item, get::get_items, update::update_item, delete::delete_item};

#[derive(Clone)]
struct AppState {
    items: Arc<Mutex<Vec<routes::models::Item>>>, // Use Arc to enable cloning
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        items: Arc::new(Mutex::new(Vec::new())),
    });
    
    let port: u16 = 8080;
    println!("Starting web server on port {}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/items", web::post().to(create_item))
            .route("/items", web::get().to(get_items))
            .route("/items", web::put().to(update_item))
            .route("/items/{id}", web::delete().to(delete_item))
    })
    .bind(("127.0.0.1", port))?
    .workers(2)
    .run()
    .await
}
