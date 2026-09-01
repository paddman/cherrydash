// SPDX-License-Identifier: AGPL-3.0-only
#![forbid(unsafe_code)]

use std::{net::SocketAddr, sync::Arc};

use anyhow::Context;
use axum::{Json, Router, extract::State, http::HeaderName, routing::get};
use chrono::{DateTime, Utc};
use clap::Parser;
use serde::Serialize;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Clone, Parser)]
#[command(
    name = "cherrydash-server",
    version,
    about = "CherryDash control plane API"
)]
struct Settings {
    #[arg(long, env = "CHERRYDASH_BIND_ADDR", default_value = "0.0.0.0:8080")]
    bind_addr: SocketAddr,

    #[arg(long, env = "CHERRYDASH_ENVIRONMENT", default_value = "development")]
    environment: String,

    #[arg(long, env = "CHERRYDASH_LOG_JSON", default_value_t = false)]
    log_json: bool,
}

#[derive(Clone)]
struct AppState {
    started_at: DateTime<Utc>,
    environment: Arc<str>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
    version: &'static str,
    current_time: DateTime<Utc>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SystemInfoResponse {
    product: &'static str,
    service: &'static str,
    version: &'static str,
    environment: String,
    started_at: DateTime<Utc>,
    uptime_seconds: i64,
    architecture: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct OverviewResponse {
    health: &'static str,
    hosts_total: u64,
    hosts_problem: u64,
    active_alerts: u64,
    events_per_second: u64,
    edge_collectors_online: u64,
    signals: Vec<&'static str>,
    capabilities: Vec<&'static str>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = Settings::parse();
    init_tracing(settings.log_json);

    let state = AppState {
        started_at: Utc::now(),
        environment: Arc::from(settings.environment),
    };

    let request_id_header = HeaderName::from_static("x-request-id");
    let app = Router::new()
        .route("/healthz", get(health))
        .route("/readyz", get(health))
        .route("/api/v1/system/info", get(system_info))
        .route("/api/v1/overview", get(overview))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(PropagateRequestIdLayer::new(request_id_header.clone()))
        .layer(SetRequestIdLayer::new(request_id_header, MakeRequestUuid));

    let listener = tokio::net::TcpListener::bind(settings.bind_addr)
        .await
        .with_context(|| format!("failed to bind {}", settings.bind_addr))?;

    tracing::info!(
        address = %settings.bind_addr,
        version = env!("CARGO_PKG_VERSION"),
        "CherryDash control plane listening"
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("control plane server stopped unexpectedly")?;

    Ok(())
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "cherrydash-server",
        version: env!("CARGO_PKG_VERSION"),
        current_time: Utc::now(),
    })
}

async fn system_info(State(state): State<AppState>) -> Json<SystemInfoResponse> {
    Json(SystemInfoResponse {
        product: "CherryDash",
        service: "cherrydash-server",
        version: env!("CARGO_PKG_VERSION"),
        environment: state.environment.to_string(),
        started_at: state.started_at,
        uptime_seconds: (Utc::now() - state.started_at).num_seconds().max(0),
        architecture: "rust-modular-control-plane",
    })
}

async fn overview() -> Json<OverviewResponse> {
    Json(OverviewResponse {
        health: "foundation",
        hosts_total: 0,
        hosts_problem: 0,
        active_alerts: 0,
        events_per_second: 0,
        edge_collectors_online: 0,
        signals: vec!["metrics", "logs", "traces", "events", "inventory"],
        capabilities: vec![
            "distributed-monitoring",
            "unified-dashboards",
            "multi-tenant-schema",
            "open-telemetry-contract",
            "prometheus-compatibility-planned",
            "automation-with-approval-planned",
        ],
    })
}

fn init_tracing(json: bool) {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("cherrydash=info,tower_http=info"));

    if json {
        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .json()
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .compact()
            .init();
    }
}

async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};

        let mut terminate =
            signal(SignalKind::terminate()).expect("failed to register SIGTERM handler");
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {},
            _ = terminate.recv() => {},
        }
    }

    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
    }

    tracing::info!("shutdown signal received");
}
