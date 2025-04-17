use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};

static mut TEMPERATURE: f32 = 0.0;
static mut HUMIDITY: f32 = 0.0;
static mut PRESSURE: f32 = 0.0;

#[derive(Deserialize, Serialize, Default, Clone)]
struct BME280 {
    temperature: f32,
    humidity: f32,
    pressure: f32,
}

#[get("/")]
async fn get_bme280() -> impl Responder {
    let mut bme280 = BME280::default();
    unsafe {
        bme280.temperature = TEMPERATURE;
        bme280.humidity = HUMIDITY;
        bme280.pressure = PRESSURE;
    }
    HttpResponse::Ok().json(bme280)
}

#[get("/set")]
async fn set_bme280(info: web::Query<BME280>) -> impl Responder {
    unsafe {
        TEMPERATURE = info.temperature;
        HUMIDITY = info.humidity;
        PRESSURE = info.pressure;
    }
    
    HttpResponse::Ok().body(
        format!("Temperature: {}, Humidity: {}, Pressure: {}", 
        info.temperature, info.humidity, info.pressure)
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("服务器已启动！");

    HttpServer::new(|| {
        let cors = Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["GET"])
        .allowed_headers(vec!["Content-Type", "Authorization"])
        .max_age(3600);

        App::new()
            .wrap(cors)
            .service(get_bme280)
            .service(set_bme280)
    })
    .bind(("0.0.0.0", 3377))?
    .run()
    .await
}