use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct TravelRuleRequest { pub amount_chf: f64, pub originator: String, pub beneficiary: String }
#[derive(Serialize)]
pub struct TravelRuleResponse { pub status: String, pub finma_ref: String, pub compliant: bool }
#[allow(dead_code)]
impl TravelRuleRequest { pub fn is_compliant(&self) -> bool { !self.originator.is_empty() && !self.beneficiary.is_empty() && self.amount_chf > 0.0 } }
