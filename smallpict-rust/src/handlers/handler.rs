use axum::{
    extract::{FromRequest, Multipart, Request, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use chrono::Utc;
use serde_json::{json, Value};
use std::sync::Arc;
use crate::{db::check_db_health, helper::smallpict::upload_smallpict, models::UploadJsonPayload, AppState};

pub async fn get_base(State(state): State<Arc<AppState>>) -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": state.config.app_name,
        "version": "1.0.0"
    }))
}

pub async fn get_health(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let is_healthy = check_db_health(&state.pool).await;

    let (status_code, db_status, status_text) = if is_healthy {
        (StatusCode::OK, "connected", "ok")
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "disconnected", "unhealthy")
    };

    (
        status_code,
        Json(json!({
            "status": status_text,
            "database": db_status,
            "timestamp": Utc::now().to_rfc3339()
        })),
    )
}

pub async fn post_upload(
    State(state): State<Arc<AppState>>,
    req: Request,
) -> impl IntoResponse {
    let content_type = req
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_lowercase();

    if content_type.contains("application/json") {
        let body_bytes = match axum::body::to_bytes(req.into_body(), usize::MAX).await {
            Ok(b) => b,
            Err(e) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "status": "error", "message": format!("Failed to read body: {}", e) })),
                );
            }
        };

        let payload: UploadJsonPayload = match serde_json::from_slice(&body_bytes) {
            Ok(p) => p,
            Err(e) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "status": "error", "message": format!("Invalid JSON: {}", e) })),
                );
            }
        };

        let base64_str = payload.image.or(payload.base64).or(payload.data);
        let filename = payload.name.or(payload.filename);

        if base64_str.is_none() {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "status": "error", "message": "Field 'image' or 'base64' is required" })),
            );
        }

        match upload_smallpict(&state.pool, &state.config, None, base64_str, filename).await {
            Ok(record) => (
                StatusCode::OK,
                Json(json!({
                    "status": "success",
                    "message": "Image uploaded successfully",
                    "data": record
                })),
            ),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "status": "error", "message": err })),
            ),
        }
    } else if content_type.contains("multipart/form-data") {
        let mut multipart = match Multipart::from_request(req, &state).await {
            Ok(m) => m,
            Err(e) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "status": "error", "message": format!("Failed to parse multipart: {}", e) })),
                );
            }
        };

        let mut file_bytes: Option<Vec<u8>> = None;
        let mut original_filename: Option<String> = None;
        let mut form_base64: Option<String> = None;

        while let Ok(Some(field)) = multipart.next_field().await {
            let name = field.name().unwrap_or_default().to_string();
            let file_name = field.file_name().map(|s| s.to_string());

            if name == "image" || name == "file" {
                if let Some(fname) = file_name {
                    original_filename = Some(fname);
                    if let Ok(bytes) = field.bytes().await {
                        file_bytes = Some(bytes.to_vec());
                    }
                } else if let Ok(text) = field.text().await {
                    form_base64 = Some(text);
                }
            } else if name == "name" || name == "filename" {
                if let Ok(text) = field.text().await {
                    original_filename = Some(text);
                }
            } else if name == "base64" {
                if let Ok(text) = field.text().await {
                    form_base64 = Some(text);
                }
            }
        }

        if file_bytes.is_none() && form_base64.is_none() {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "status": "error", "message": "No image file or base64 data found in multipart request" })),
            );
        }

        match upload_smallpict(&state.pool, &state.config, file_bytes, form_base64, original_filename).await {
            Ok(record) => (
                StatusCode::OK,
                Json(json!({
                    "status": "success",
                    "message": "Image uploaded successfully",
                    "data": record
                })),
            ),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "status": "error", "message": err })),
            ),
        }
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "status": "error", "message": "Unsupported Content-Type. Use multipart/form-data or application/json" })),
        )
    }
}
