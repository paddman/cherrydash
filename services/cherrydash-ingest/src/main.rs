#![forbid(unsafe_code)]

use std::{
    net::SocketAddr,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

use anyhow::Context;
use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, State},
    http::{HeaderMap, HeaderName, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use chrono::Utc;
use cherrydash_core::{TelemetryEnvelope, TelemetryInput};
use clap::Parser;
use serde::Serialize;
use tokio::{
    fs::{File, OpenOptions},
    io::AsyncWriteExt,
    sync::Mutex,
};
use tower_http::{
    cors::CorsLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use tracing_subscriber::EnvFilter;

const TENANT_HEADER: &str = "x-cherrydash-tenant-id";

#[derive(Debug, Clone, Parser)]
#[command(name = "cherrydash-ingest", version, about = "CherryDash telemetry ingestion gateway")]
struct Settings {
    #[arg(long, env = "CHERRYDASH_INGEST_BIND_ADDR", default_value = "0.0.0.0:8081")]
    bind_addr: SocketAddr,

    #[arg(
        long,
        env = "CHERRYDASH_INGEST_WAL_PATH",
        default_value = "/var/lib/cherrydash/ingest/events.ndjson"
    )]
    wal_path: PathBuf,

    #[arg(long, env = "CHERRYDASH_INGEST_MAX_BODY_BYTES", default_value_t = 1_048_576)]
    max_body_bytes: usize,

    #[arg(long, env = "CHERRYDASH_INGEST_SYNC_WRITES", default_value_t = false)]
    sync_writes: bool,

    #[arg(long, env = "CHERRYDASH_LOG_JSON", default_value_t = false)]
    log_json: bool,
}

#[derive(Clone)]
struct AppState {
    writer: Arc<Mutex<File>>,
    wal_path: Arc<PathBuf>,
    sync_writes: bool,
    accepted_total: Arc<AtomicU64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
    version: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct IngestStatusResponse {
    status: &'static str,
    accepted_total: u64,
    transport: &'static str,
    durable_wal: bool,
    wal_path: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AcceptedResponse {
    status: &'static str,
    event_id: String,
    signal: String,
    received_at: chrono::DateTime<Utc>,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: &'static str,
    message: String,
}

#[derive(Debug)]
enum ApiError {
    BadRequest(String),
    Internal,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error, message) = match self {
            Self::BadRequest(message) => (StatusCode::BAD_REQUEST, "bad_request", message),
            Self::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "telemetry could not be persisted".to_owned(),
            ),
        };

        (status, Json(ErrorResponse { error, message })).into_response()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = Settings::parse();
    init_tracing(settings.log_json);

    if let Some(parent) = settings
        .wal_path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
    {
        tokio::fs::create_dir_all(parent)
            .await
            .with_context(|| format!("failed to create WAL directory {}", parent.display()))?;
    }

    let writer = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&settings.wal_path)
        .await
        .with_context(|| format!("failed to open WAL {}", settings.wal_path.display()))?;

    let state = AppState {
        writer: Arc::new(Mutex::new(writer)),
        wal_path: Arc::new(settings.wal_path.clone()),
        sync_writes: settings.sync_writes,
        accepted_total: Arc::new(AtomicU64::new(0)),
    };

    let request_id_header = HeaderName::from_static("x-request-id");
    let app = Router::new()
        .route("/healthz", get(health))
        .route("/readyz", get(health))
        .route("/api/v1/ingest/status", get(ingest_status))
        .route("/api/v1/events", post(ingest_event))
        .with_state(state)
        .layer(DefaultBodyLimit::max(settings.max_body_bytes))
        .layer(TraceLayer::new_for_http())
        .layer(PropagateRequestIdLayer::new(request_id_header.clone()))
        .layer(SetRequestIdLayer::new(
            request_id_header,
            MakeRequestUuid,
        ))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind(settings.bind_addr)
        .await
        .with_context(|| format!("failed to bind {}", settings.bind_addr))?;

    tracing::info!(
        address = %settings.bind_addr,
        wal = %settings.wal_path.display(),
        sync_writes = settings.sync_writes,
        "CherryDash ingestion gateway listening"
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("ingestion gateway stopped unexpectedly")?;

    Ok(())
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "cherrydash-ingest",
        version: env!("CARGO_PKG_VERSION"),
    })
}

async fn ingest_status(State(state): State<AppState>) -> Json<IngestStatusResponse> {
    Json(IngestStatusResponse {
        status: "ready",
        accepted_total: state.accepted_total.load(Ordering::Relaxed),
        transport: "append-only-local-wal",
        durable_wal: true,
        wal_path: state.wal_path.display().to_string(),
    })
}

async fn ingest_event(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<TelemetryInput>,
) -> Result<impl IntoResponse, ApiError> {
    let tenant_id = headers
        .get(TENANT_HEADER)
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| ApiError::BadRequest(format!("missing {TENANT_HEADER} header")))?;

    let received_at = Utc::now();
    let envelope = TelemetryEnvelope::from_input(tenant_id, input, received_at)
        .map_err(|error| ApiError::BadRequest(error.to_string()))?;

    let mut encoded = serde_json::to_vec(&envelope).map_err(|error| {
        tracing::error!(%error, "failed to encode telemetry envelope");
        ApiError::Internal
    })?;
    encoded.push(b'\n');

    let mut writer = state.writer.lock().await;
    writer.write_all(&encoded).await.map_err(|error| {
        tracing::error!(%error, "failed to append telemetry to WAL");
        ApiError::Internal
    })?;
    writer.flush().await.map_err(|error| {
        tracing::error!(%error, "failed to flush telemetry WAL");
        ApiError::Internal
    })?;

    if state.sync_writes {
        writer.sync_data().await.map_err(|error| {
            tracing::error!(%error, "failed to sync telemetry WAL");
            ApiError::Internal
        })?;
    }
    drop(writer);

    state.accepted_total.fetch_add(1, Ordering::Relaxed);

    Ok((
        StatusCode::ACCEPTED,
        Json(AcceptedResponse {
            status: "accepted",
            event_id: envelope.event_id.to_string(),
            signal: envelope.signal.to_string(),
            received_at,
        }),
    ))
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

        let mut terminate = signal(SignalKind::terminate())
            .expect("failed to register SIGTERM handler");
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
