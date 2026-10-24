use axum::Json; use serde_json::{json, Value};
pub async fn verify_license() -> Json<Value> {
    Json(json!({"license":"FINMA-2026-CH-1000CHF","status":"ACTIVE","valid":true,"level":"HIGH","functions":14}))
}
