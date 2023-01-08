use actix_web::{get, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

mod models;
mod services;


struct AppState{
    users_list: Mutex<Vec<User>>,
    groups_list: Mutex<Vec<Group>>
}

#[derive(Serialize, Deserialize, Clone)]
struct User{
    id: i32,
    name: String
}

#[derive(Serialize, Deserialize, Clone)]
struct Group{
    id: i32,
    name: String,
    is_open: bool,
    admins_list: Vec<i32>,
    members_list: Vec<i32>,
    secret_santa_list: Vec<i32>
}

#[get("/")]
async fn index() -> String{
    "I'm alive".to_string()
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let app_data = web::Data::new(AppState{
        users_list : Mutex::new(vec![]),
        groups_list : Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .configure(services::users_config)
    })
        .bind(("127.0.0.1", 8079))?
        .run()
        .await
}