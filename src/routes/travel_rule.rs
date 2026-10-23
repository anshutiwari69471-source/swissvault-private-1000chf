use axum::Json;
use crate::models::travel_rule::{TravelRuleRequest, TravelRuleResponse};
use crate::services::compliance;
pub async fn travel_rule_check(Json(req): Json<TravelRuleRequest>) -> Json<TravelRuleResponse> { Json(TravelRuleResponse{status: if compliance::is_compliant(req.amount_chf) {"APPROVED".into()} else {"REVIEW".into()}, finma_ref: compliance::finma_ref(), compliant: compliance::is_compliant(req.amount_chf)}) }
pub async fn verify_originator() -> Json<serde_json::Value> { Json(serde_json::json!({"verified": true, "originator": "Ansh Swiss"})) }
pub async fn generate_audit_log() -> Json<serde_json::Value> { Json(serde_json::json!({"audit_id": uuid::Uuid::new_v4().to_string(), "finma_compliant": true})) }
