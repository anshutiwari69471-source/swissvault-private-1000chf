use serde::{Deserialize, Serialize};
#[derive(Deserialize)] pub struct KycRequest { pub name: String, pub country: String }
#[derive(Serialize)] pub struct KycStatus { pub status: String, pub risk: u8 }
