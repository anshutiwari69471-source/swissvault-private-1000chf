use serde::{Deserialize, Serialize};
#[derive(Deserialize)] pub struct TravelRuleRequest { pub amount_chf: f64, pub originator: String, pub beneficiary: String }
#[derive(Serialize)] pub struct TravelRuleResponse { pub status: String, pub finma_ref: String, pub compliant: bool }
