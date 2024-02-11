pub mod fetcher;
#[allow(unused_imports)]
use crate::fetcher::*;

pub mod error;

use crate::error::AppError;
#[allow(unused_imports)]
use axum::{
    extract::FromRequest,
    extract::Query,
    extract::{rejection::ExtensionRejection, Extension},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};

#[allow(unused_imports)]
use axum_macros::{debug_handler, FromRequestParts};

use serde::Deserialize;

#[allow(unused_imports)]
use std::{collections::HashMap, net::SocketAddr};

async fn index() -> &'static str {
    "Index"
}

#[derive(Deserialize, FromRequestParts)]

pub struct WeatherQuery {
    #[from_request(via(Extension))]
    pub city: String,
}

impl IntoResponse for WeatherQuery {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong").into_response()
    }
}

#[debug_handler]
async fn weather(Query(params): Query<WeatherQuery>) -> Result<WeatherDisplay, AppError> {
    let lat_long = fetch_lat_long(&params.city).await?;

    let weather = fetch_weather(lat_long).await?;
    Ok(WeatherDisplay::new(params.city, weather))
}

async fn stats() -> &'static str {
    "Stats"
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let app = Router::new()
        .route("/", get(index))
        .route("/weather", get(weather))
        .route("/stats", get(stats));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
