use axum::{Json, http::StatusCode};
use crate::models::task::Task;
use uuid::Uuid;
pub async fn get_tasks() -> Json<Vec<Task>> { Json(vec![Task{id: Uuid::new_v4().to_string(), title: "FINMA Review".into(), status: "pending".into()}]) }
pub async fn create_task() -> (StatusCode, Json<Task>) { (StatusCode::CREATED, Json(Task{id: Uuid::new_v4().to_string(), title: "New Task".into(), status: "created".into()})) }
pub async fn update_task() -> Json<Task> { Json(Task{id: "1".into(), title: "Updated".into(), status: "done".into()}) }
pub async fn delete_task() -> StatusCode { StatusCode::NO_CONTENT }
