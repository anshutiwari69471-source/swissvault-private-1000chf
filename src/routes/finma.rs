use axum::Json; use serde_json::{json, Value};
pub async fn demo_report() -> Json<Value> {
    Json(json!({"demo_ref": "EDU-DEMO-1000-2026", "threshold_example_chf": 1000, "status": "SIMULATED_FOR_DEMO", "risk": "LOW - For Learning Only", "note": "This is an educational simulation, not real regulatory reporting"}))
}
