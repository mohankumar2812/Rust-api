mod api;
mod models;
mod repository;

use actix_web::{web::Data, App, HttpServer};
use actix_web::http::StatusCode;
use api::user_api::{create_user, delete_user, get_all_users, get_user, update_user, login_user};
use repository::mongodb_repo::MongoRepo;
use actix_cors::Cors;

pub mod constant;
pub mod error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    // let cors = Cors::default()
    //         .allowed_origin("http://localhost:3000")
    //         .allowed_methods(vec!["POST", "PUT", "PATCH", "GET", "OPTIONS", "HEAD"]);

    HttpServer::new(move || {

        let cors = Cors::default()
        .allowed_origin("http://localhost:3000")
        .allowed_methods(vec!["GET", "POST"])
        .allowed_header(actix_web::http::header::ACCEPT)
        .allowed_header(actix_web::http::header::CONTENT_TYPE) 
        .max_age(3600);
        
        App::new().wrap(cors)
            .app_data(db_data.clone())
            .service(create_user)
            .service(login_user)
            .service(get_user)
            .service(update_user)
            .service(delete_user)
            .service(get_all_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
