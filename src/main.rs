//external imports
use actix_cors::Cors;
use actix_web::{http, middleware, App, HttpServer};
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use std::env;
use api_service::ApiService;

//referencia de modulos externos

mod api_router;
mod api_service;

//constructor api service

pub struct ServiceManager {
    api: ApiService,
}

//implementacion api service

impl ServiceManager {
    pub fn new(api: ApiService) -> Self {
        ServiceManager { api }
    }
}

//constructor service manager

pub struct AppState {
    service_manager: ServiceManager,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //inicializa env

    dotenv().ok();

    //inicializa el logger de middleware

    env::set_var("RUST_LOG", " actix_web=debug,actix_server=info");
    env_logger::init();

    //pasa el string de conexion a estructura de opciones

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL no esta en el archivo env");
    let client_options = ClientOptions::parse(&database_url).unwrap();

    //referencia a mongodb

    let client = Client::with_options(client_options).unwrap();

    //referencia a base de datos

    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME no esta en el archivo .env");
    let db = client.database(&database_name);

    //referencia a la datos
    let user_collection_name = env::var("USER_COLLECTION_NAME").expect("COLLECTION_NAME no esta en el archivo .env");
    let user_collection = db.collection(&user_collection_name);

    //obtener url del server

    let server_url = env::var("SERVER_URL").expect("SERVER_URL no esta en el archivo .env");

    //abre el servidor

    HttpServer::new(move || {
        let user_service_worker = ApiService::new(user_collection.clone());
        let service_manager = ServiceManager::new(user_service_worker);
        //cors
        let cors_middleware = Cors::new()
            .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600)
            .finish();
        
        //inicializar el servidor http
        App::new()
            .wrap(cors_middleware)
            .wrap(middleware::Logger::default())
            .data(AppState { service_manager })
            .configure(api_router::init)
        
    })
    .bind(server_url)?
    .run()
    .await
}