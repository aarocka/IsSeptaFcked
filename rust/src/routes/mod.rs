use axum::{
    extract::Host,
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    routing::get,
    Router,
};
use serde_json::json;
use tera::{Context, Tera};

use crate::septa;
use crate::sfw;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/api", get(api_root))
        .route("/api/status", get(api_status))
        .route("/api/rr", get(api_rr))
        .route("/api/rr/status", get(api_rr_status))
        .route("/api/rr/raw_data", get(api_rr_raw))
        .route("/api/bus", get(api_bus))
        .route("/api/bus/status", get(api_bus_status))
        .route("/api/bus/raw_data", get(api_bus_raw))
        .route("/faq", get(faq))
        .route("/echo", get(echo))
}

async fn index(Host(host): Host) -> impl IntoResponse {
    let is_sfw = sfw::is_sfw(&host);
    let rr_data = septa::rr::get_data();
    let bus_data = septa::bus::get_data();

    let mut tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Template error: {}", e);
            return Html("<h1>Template error</h1>".to_string());
        }
    };

    let mut context = Context::new();
    
    let mut title = "Is SEPTA Fucked?".to_string();
    let mut rr_status = rr_data.status.status.clone();
    let mut rr_message = rr_data.status.message.clone();
    let mut bus_status = bus_data.status.status.clone();
    let mut bus_message = bus_data.status.message.clone();

    if is_sfw {
        title = sfw::filter(&title);
        rr_status = sfw::filter(&rr_status);
        rr_message = sfw::filter(&rr_message);
        bus_status = sfw::filter(&bus_status);
        bus_message = sfw::filter(&bus_message);
    }

    context.insert("title", &title);
    context.insert("rr_status", &rr_status);
    context.insert("rr_status_class", &rr_data.status.css_class);
    context.insert("rr_status_time", &rr_data.time);
    context.insert("rr_late", &rr_data.status.late);
    context.insert("rr_message", &rr_message);
    context.insert("bus_status", &bus_status);
    context.insert("bus_status_class", &bus_data.status.css_class);
    context.insert("bus_status_time", &bus_data.time);
    context.insert("bus_suspended", &bus_data.status.suspended);
    context.insert("bus_message", &bus_message);
    context.insert("is_sfw", &is_sfw);

    match tera.render("index.html", &context) {
        Ok(html) => Html(html),
        Err(e) => {
            tracing::error!("Template rendering error: {}", e);
            Html("<h1>Error rendering template</h1>".to_string())
        }
    }
}

async fn faq(Host(host): Host) -> impl IntoResponse {
    let is_sfw = sfw::is_sfw(&host);
    
    let mut tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Template error: {}", e);
            return Html("<h1>Template error</h1>".to_string());
        }
    };

    let mut context = Context::new();
    
    let mut title = "Is SEPTA Fucked? - FAQ".to_string();
    if is_sfw {
        title = sfw::filter(&title);
    }

    context.insert("title", &title);
    context.insert("is_sfw", &is_sfw);

    match tera.render("faq.html", &context) {
        Ok(html) => Html(html),
        Err(e) => {
            tracing::error!("Template rendering error: {}", e);
            Html("<h1>Error rendering template</h1>".to_string())
        }
    }
}

async fn echo() -> impl IntoResponse {
    Html("<html><body><h1>Echo test successful!</h1></body></html>")
}

async fn api_root() -> impl IntoResponse {
    Json(json!({
        "endpoints": {
            "status": "/api/status",
            "regional_rail": "/api/rr",
            "regional_rail_status": "/api/rr/status",
            "regional_rail_raw": "/api/rr/raw_data",
            "bus": "/api/bus",
            "bus_status": "/api/bus/status",
            "bus_raw": "/api/bus/raw_data"
        }
    }))
}

async fn api_status() -> impl IntoResponse {
    let rr_data = septa::rr::get_data();
    let bus_data = septa::bus::get_data();
    
    Json(json!({
        "regional_rail": {
            "status": rr_data.status.status,
            "num_trains": rr_data.num,
            "last_updated": rr_data.time
        },
        "bus": {
            "status": bus_data.status.status,
            "num_routes": bus_data.num,
            "last_updated": bus_data.time
        }
    }))
}

async fn api_rr() -> impl IntoResponse {
    let data = septa::rr::get_data();
    Json(data)
}

async fn api_rr_status() -> impl IntoResponse {
    let data = septa::rr::get_data();
    Json(data.status)
}

async fn api_rr_raw() -> impl IntoResponse {
    let data = septa::rr::get_raw_data();
    Json(data)
}

async fn api_bus() -> impl IntoResponse {
    let data = septa::bus::get_data();
    Json(data)
}

async fn api_bus_status() -> impl IntoResponse {
    let data = septa::bus::get_data();
    Json(data.status)
}

async fn api_bus_raw() -> impl IntoResponse {
    let data = septa::bus::get_raw_data();
    Json(data)
}
