//! Weather Mock API implemented in Rust using Actix Web.
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use tracing::{info, instrument};
use weather_data::{get_forecast, WeatherForecast};

mod weather_data;

/// Represents coordinates of a geographical location
#[derive(Deserialize, Debug)]
pub struct Coordinates {
    /// Latitude
    lat: f64,
    /// Longitude
    lon: f64,
}

#[instrument]
async fn get_forecasts(coordinates: web::Query<Coordinates>) -> impl Responder {
    let lat = coordinates.lat;
    let lon = coordinates.lon;

    info!("Request received: lat = {}, lon = {}", lat, lon);

    let mock_data: Vec<WeatherForecast> = get_forecast();
    info!("Responding with mock weather data");

    HttpResponse::Ok().json(mock_data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up tracing
    tracing_subscriber::fmt::init();

    info!("Starting the server...");

    // Start the server
    HttpServer::new(|| {
        App::new()
            .wrap(tracing_actix_web::TracingLogger::default())
            .route("/forecasts", web::get().to(get_forecasts))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
