use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ImageRecord {
    pub id: String,
    pub filename: Option<String>,
    pub url: Option<String>,
    pub size: Option<i64>,
    pub size_origin: Option<i64>,
    pub mime_type: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct UploadJsonPayload {
    pub image: Option<String>,
    pub base64: Option<String>,
    pub data: Option<String>,
    pub name: Option<String>,
    pub filename: Option<String>,
}
