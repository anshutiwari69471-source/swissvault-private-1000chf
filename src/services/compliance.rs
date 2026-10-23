use uuid::Uuid;
pub fn finma_ref() -> String { format!("FINMA-2026-{}", &Uuid::new_v4().to_string()[..8].to_uppercase()) }
pub fn risk_score(country: &str) -> u8 { if country.to_lowercase()=="ch" {10} else {50} }
pub fn is_compliant(amount: f64) -> bool { amount >= crate::config::FINMA_THRESHOLD }
