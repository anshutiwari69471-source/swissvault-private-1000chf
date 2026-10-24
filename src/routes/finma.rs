use axum::Json; use serde_json::{json, Value};
pub async fn finma_report() -> Json<Value> {
    Json(json!({"finma_ref":"FINMA-1000-2026","threshold_chf":1000,"status":"REPORTED","risk":"LOW","kyc_required":true}))
}
