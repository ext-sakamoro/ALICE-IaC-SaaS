// ALICE-IaC-SaaS core-engine
// License: AGPL-3.0-or-later

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;
use uuid::Uuid;

#[derive(Debug, Default, Clone, Serialize)]
struct Stats {
    total_requests: u64,
    plan_requests: u64,
    apply_requests: u64,
    state_requests: u64,
    monitor_requests: u64,
}

#[derive(Clone)]
struct AppState {
    stats: Arc<Mutex<Stats>>,
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
    version: &'static str,
}

#[derive(Deserialize)]
struct PlanRequest {
    workspace: String,
    template: String,
    variables: Option<serde_json::Value>,
}

#[derive(Serialize)]
struct PlanResponse {
    id: String,
    workspace: String,
    status: &'static str,
    add: u32,
    change: u32,
    destroy: u32,
}

#[derive(Deserialize)]
struct ApplyRequest {
    plan_id: String,
    auto_approve: Option<bool>,
}

#[derive(Serialize)]
struct ApplyResponse {
    id: String,
    plan_id: String,
    status: &'static str,
    resources_applied: u32,
}

#[derive(Serialize)]
struct StateResponse {
    id: String,
    workspace: &'static str,
    resources: u32,
    last_modified: &'static str,
}

#[derive(Serialize)]
struct MonitorResponse {
    id: String,
    healthy_resources: u32,
    degraded_resources: u32,
    total_resources: u32,
    status: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "alice-iac-core-engine",
        version: "0.1.0",
    })
}

async fn iac_plan(
    State(state): State<AppState>,
    Json(req): Json<PlanRequest>,
) -> Result<Json<PlanResponse>, StatusCode> {
    let mut stats = state.stats.lock().unwrap();
    stats.total_requests += 1;
    stats.plan_requests += 1;
    info!("iac/plan workspace={}", req.workspace);
    Ok(Json(PlanResponse {
        id: Uuid::new_v4().to_string(),
        workspace: req.workspace,
        status: "planned",
        add: 3,
        change: 1,
        destroy: 0,
    }))
}

async fn iac_apply(
    State(state): State<AppState>,
    Json(req): Json<ApplyRequest>,
) -> Result<Json<ApplyResponse>, StatusCode> {
    let mut stats = state.stats.lock().unwrap();
    stats.total_requests += 1;
    stats.apply_requests += 1;
    info!("iac/apply plan_id={}", req.plan_id);
    Ok(Json(ApplyResponse {
        id: Uuid::new_v4().to_string(),
        plan_id: req.plan_id,
        status: "applied",
        resources_applied: 4,
    }))
}

async fn iac_state(State(state): State<AppState>) -> Json<StateResponse> {
    let mut stats = state.stats.lock().unwrap();
    stats.total_requests += 1;
    stats.state_requests += 1;
    Json(StateResponse {
        id: "state-default",
        workspace: "default",
        resources: 12,
        last_modified: "2026-03-09T00:00:00Z",
    })
}

async fn iac_monitor(State(state): State<AppState>) -> Json<MonitorResponse> {
    let mut stats = state.stats.lock().unwrap();
    stats.total_requests += 1;
    stats.monitor_requests += 1;
    Json(MonitorResponse {
        id: "monitor-default",
        healthy_resources: 11,
        degraded_resources: 1,
        total_resources: 12,
        status: "degraded",
    })
}

async fn iac_stats(State(state): State<AppState>) -> Json<Stats> {
    let stats = state.stats.lock().unwrap().clone();
    Json(stats)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    let state = AppState {
        stats: Arc::new(Mutex::new(Stats::default())),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/iac/plan", post(iac_plan))
        .route("/api/v1/iac/apply", post(iac_apply))
        .route("/api/v1/iac/state", get(iac_state))
        .route("/api/v1/iac/monitor", get(iac_monitor))
        .route("/api/v1/iac/stats", get(iac_stats))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9141").await.unwrap();
    info!("alice-iac-core-engine listening on :9141");
    axum::serve(listener, app).await.unwrap();
}
