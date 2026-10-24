use axum::{Json, response::IntoResponse};
use serde_json::json;

pub async fn demo_report() -> impl IntoResponse {
    Json(json!({
        "status": "success",
        "message": "SwissVault Educational Demo - For Learning Only",
        "demo_data": {
            "vault_balance": "1000.00 DEMO",
            "transactions": 5,
            "note": "This is simulated data only"
        }
    }))
}
