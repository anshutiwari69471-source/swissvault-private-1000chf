use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Clone)]
pub struct KycRequest { pub name: String, pub country: String }
#[derive(Serialize)]
pub struct KycStatus { pub status: String, pub risk: u8 }
#[allow(dead_code)]
impl KycRequest { pub fn validate(&self) -> bool { !self.name.is_empty() && self.country == "CH" } }
