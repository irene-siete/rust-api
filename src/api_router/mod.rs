use crate::api_service::Data;
use actix_web::{delete, get, post, web, HttpResponse, Responder};

#[get("/get-all")]
async fn get_all_json(app_data: web::Data<crate::AppState>) -> impl Responder {
    let action = app_data.service_manager.api.get_json();
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            println!("error obtieniendo, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/get-by/{param}")]
async fn get_user_email(app_data: web::Data<crate::AppState>, param: web::Path<String>) -> impl Responder {
    let action = app_data.service_manager.api.get_by(&param);
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            println!("error obtieniendo, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/add")]
async fn add_user(app_data: web::Data<crate::AppState>, data: web::Json<Data>) -> impl Responder {
    let action = app_data.service_manager.api.create(&data);
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result.inserted_id),
        Err(e) => {
            println!("error obtieniendo, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/update/{param}")]
async fn update_user(app_data: web::Data<crate::AppState>, data: web::Json<Data>, param: web::Path<String>) -> impl Responder {
    let action = app_data.service_manager.api.update(&data, &param);
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result.modified_count),
        Err(e) => {
            println!("error obtieniendo, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[delete("/delete")]
async fn delete_user(app_data: web::Data<crate::AppState>, data: web::Json<Data>) -> impl Responder {
    let action = app_data.service_manager.api.delete(&data.title);
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result.deleted_count),
        Err(e) => {
            println!("error obtieniendo, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

//funcion que sera llamada en la nueva aplicacion para configurar las rutas para este modulo

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user_email);
    cfg.service(add_user);
    cfg.service(update_user);
    cfg.service(delete_user);
    cfg.service(get_all_json);
}