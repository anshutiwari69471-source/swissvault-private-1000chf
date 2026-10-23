use axum::Json;
use crate::models::kyc::{KycRequest, KycStatus};
use crate::services::compliance;
pub async fn kyc_check(Json(req): Json<KycRequest>) -> Json<KycStatus> { Json(KycStatus{status: "VERIFIED".into(), risk: compliance::risk_score(&req.country)}) }
pub async fn get_kyc_status() -> Json<KycStatus> { Json(KycStatus{status: "ACTIVE".into(), risk: 10}) }
