pub mod fetcher;

#[allow(unused_imports)]
use crate::fetcher::*;

pub mod error;

use crate::error::AppError;
#[allow(unused_imports)]
use axum::{
    extract::{
        rejection::{ExtensionRejection, JsonRejection},
        Extension, FromRequest, Query,
    },
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};

#[allow(unused_imports)]
use axum_macros::FromRequest;

#[allow(unused_imports)]
use serde_json::json;

#[allow(unused_imports)]
use axum_macros::{debug_handler, FromRequestParts};

use serde::Deserialize;

#[allow(unused_imports)]
use std::{collections::HashMap, net::SocketAddr};

use askama::Template;

// async fn index() -> &'static str {
//     "Index"
// }

#[derive(Template)]
#[template(path = "index.html")]
#[derive(Deserialize)]
struct IndexTemplate;

#[debug_handler]
async fn index() -> IndexTemplate {
    IndexTemplate
}

impl IntoResponse for IndexTemplate {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong").into_response()
    }
}

#[derive(Deserialize)]
pub struct WeatherQuery {
    pub city: String,
}

impl IntoResponse for WeatherQuery {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong").into_response()
    }
}

// impl<B> FromRequest<B> for Query<WeatherQuery>
// where
//     B: Send,
// {
//     type Rejection = YourRejectionType;

//     fn from_request(req: &mut Request, _: &mut Context) -> Poll<Result<Self, Self::Rejection>> {
//         // Extract the `WeatherQuery` from the request here
//         // For example, you can extract it from the query parameters
//         if let Some(query) = req.uri().query() {
//             if let Ok(weather_query) = serde_urlencoded::from_str(query) {
//                 return Poll::Ready(Ok(Query(weather_query)));
//             }
//         }
//         // If extraction fails, return a rejection
//         Poll::Ready(Err(YourRejectionType))
//     }
// }

// impl From<JsonRejection> for WeatherQuery {
//     fn from(rejection: JsonRejection) -> Self {
//         let response = (
//             StatusCode::INTERNAL_SERVER_ERROR,
//             axum::Json(json!({"error": rejection.to_string()})),
//         )
//             .into_response();

//         WeatherQuery { city: response }
//     }
// }

// #[debug_handler]
// async fn weather(Query(params): Query<WeatherQuery>) -> Result<WeatherDisplay, AppError> {
//     let lat_long = fetch_lat_long(&params.city).await?;

//     let weather = fetch_weather(lat_long).await?;
//     Ok(WeatherDisplay::new(params.city, weather))
// }

#[debug_handler]
async fn weather(Query(params): Query<WeatherQuery>) -> Result<WeatherDisplay, AppError> {
    let lat_long = fetch_lat_long(&params.city).await?;
    let weather = fetch_weather(lat_long).await?;
    Ok(WeatherDisplay::new(params.city, weather))
}

// async fn weather(Query(params): Query<WeatherQuery>) -> Result<String, AppError> {
//     let lat_long = fetch_lat_long(&params.city).await?;
//     let weather = fetch_weather(lat_long).await?;
//     let display = WeatherDisplay::new(params.city, weather);
//     Ok(format!("{:?}", display))
// }

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
