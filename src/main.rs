mod config; mod models; mod routes; mod services;
use axum::{routing::get, Router};
async fn health() -> String { format!("SwissVault Educational Demo v{} LIVE - For Learning Only", config::VERSION) }
#[tokio::main]

async fn main() {
    let app = Router::new()
        .route("/", get(health))
        .route("/health", get(health))
        .route("/api/tasks", get(routes::tasks::get_tasks).post(routes::tasks::create_task))
        .route("/api/tasks/:id", axum::routing::put(routes::tasks::update_task).delete(routes::tasks::delete_task))
        .route("/api/kyc/check", axum::routing::post(routes::kyc::kyc_check))      .route("/api/kyc/status", get(routes::kyc::get_kyc_status))
        .route("/api/travel-rule", axum::routing::post(routes::travel_rule::travel_rule_check))
        .route("/api/travel-rule/verify", get(routes::travel_rule::verify_originator))
        .route("/api/travel-rule/audit", get(routes::travel_rule::generate_audit_log))
        .route("/api/license/verify", get(routes::license::verify_license))
        .route("/api/audit/export", get(routes::audit::export_audit))
        .route("/api/finma/report", get(routes::finma::finma_report));
    println!("SwissVault BEST HIGH LEVEL 14 FUNCTIONS LIVE on 0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
