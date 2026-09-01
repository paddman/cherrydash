// SPDX-License-Identifier: AGPL-3.0-only
#![forbid(unsafe_code)]

use std::{
    net::SocketAddr,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

use anyhow::{Context, bail};
use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, State},
    http::{HeaderMap, HeaderName, StatusCode, header::AUTHORIZATION},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use cherrydash_core::{TelemetryEnvelope, TelemetryInput, validate_tenant_id};
use chrono::Utc;
use clap::Parser;
use serde::{Deserialize, Serialize};
use tokio::{
    fs::{File, OpenOptions},
    io::AsyncWriteExt,
    sync::Mutex,
};
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use tracing_subscriber::EnvFilter;

const DEVELOPMENT_TOKEN: &str = "development-only-change-me";

#[derive(Debug, Clone, Parser)]
#[command(
    name = "cherrydash-ingest",
    version,
    about = "CherryDash telemetry ingestion gateway"
)]
struct Settings {
    #[arg(
        long,
        env = "CHERRYDASH_INGEST_BIND_ADDR",
        default_value = "0.0.0.0:8081"
    )]
    bind_addr: SocketAddr,

    #[arg(
        long,
        env = "CHERRYDASH_INGEST_WAL_PATH",
        default_value = "/var/lib/cherrydash/ingest/events.ndjson"
    )]
    wal_path: PathBuf,

    #[arg(
        long,
        env = "CHERRYDASH_INGEST_MAX_BODY_BYTES",
        default_value_t = 1_048_576
    )]
    max_body_bytes: usize,

    #[arg(long, env = "CHERRYDASH_INGEST_SYNC_WRITES", default_value_t = false)]
    sync_writes: bool,

    #[arg(long, env = "CHERRYDASH_ENVIRONMENT", default_value = "development")]
    environment: String,

    #[arg(
        long,
        env = "CHERRYDASH_INGEST_KEYS_JSON",
        default_value = r#"[{"tenant_id":"default","token":"development-only-change-me"}]"#
    )]
    credentials_json: String,

    #[arg(long, env = "CHERRYDASH_LOG_JSON", default_value_t = false)]
    log_json: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct IngestCredential {
    tenant_id: String,
    token: String,
}

#[derive(Clone)]
struct AppState {
    writer: Arc<Mutex<File>>,
    sync_writes: bool,
    accepted_total: Arc<AtomicU64>,
    credentials: Arc<Vec<IngestCredential>>,
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
    acknowledgement_mode: &'static str,
    durable_wal: bool,
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
    Unauthorized,
    Internal,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error, message) = match self {
            Self::BadRequest(message) => (StatusCode::BAD_REQUEST, "bad_request", message),
            Self::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "unauthorized",
                "a valid bearer token is required".to_owned(),
            ),
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

    let credentials = parse_credentials(&settings)?;

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
        sync_writes: settings.sync_writes,
        accepted_total: Arc::new(AtomicU64::new(0)),
        credentials: Arc::new(credentials),
    };

    if !settings.sync_writes {
        tracing::warn!(
            "ingest acknowledgements use flush mode and are not crash-durable; enable CHERRYDASH_INGEST_SYNC_WRITES for durable acknowledgement"
        );
    }

    let request_id_header = HeaderName::from_static("x-request-id");
    let app = Router::new()
        .route("/healthz", get(health))
        .route("/readyz", get(readiness))
        .route("/api/v1/ingest/status", get(ingest_status))
        .route("/api/v1/events", post(ingest_event))
        .with_state(state.clone())
        .layer(DefaultBodyLimit::max(settings.max_body_bytes))
        .layer(TraceLayer::new_for_http())
        .layer(PropagateRequestIdLayer::new(request_id_header.clone()))
        .layer(SetRequestIdLayer::new(request_id_header, MakeRequestUuid));

    let listener = tokio::net::TcpListener::bind(settings.bind_addr)
        .await
        .with_context(|| format!("failed to bind {}", settings.bind_addr))?;

    tracing::info!(
        address = %settings.bind_addr,
        wal = %settings.wal_path.display(),
        sync_writes = settings.sync_writes,
        credential_count = state.credentials.len(),
        "CherryDash ingestion gateway listening"
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("ingestion gateway stopped unexpectedly")?;

    Ok(())
}

fn parse_credentials(settings: &Settings) -> anyhow::Result<Vec<IngestCredential>> {
    let credentials: Vec<IngestCredential> = serde_json::from_str(&settings.credentials_json)
        .context("CHERRYDASH_INGEST_KEYS_JSON must be a JSON array of tenant_id/token objects")?;

    if credentials.is_empty() {
        bail!("at least one ingest credential is required");
    }

    for credential in &credentials {
        validate_tenant_id(&credential.tenant_id).with_context(|| {
            format!(
                "invalid tenant id in ingest credential: {}",
                credential.tenant_id
            )
        })?;

        if credential.token.len() < 24 {
            bail!("ingest tokens must contain at least 24 bytes");
        }
    }

    if !settings.environment.eq_ignore_ascii_case("development")
        && credentials
            .iter()
            .any(|credential| credential.token == DEVELOPMENT_TOKEN)
    {
        bail!("the development ingest token is forbidden outside the development environment");
    }

    Ok(credentials)
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "cherrydash-ingest",
        version: env!("CARGO_PKG_VERSION"),
    })
}

async fn readiness(State(state): State<AppState>) -> Response {
    let writer = state.writer.lock().await;
    match writer.metadata().await {
        Ok(_) => Json(HealthResponse {
            status: "ready",
            service: "cherrydash-ingest",
            version: env!("CARGO_PKG_VERSION"),
        })
        .into_response(),
        Err(error) => {
            tracing::error!(%error, "WAL is not writable/readable for readiness check");
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(ErrorResponse {
                    error: "not_ready",
                    message: "the ingestion WAL is unavailable".to_owned(),
                }),
            )
                .into_response()
        }
    }
}

async fn ingest_status(State(state): State<AppState>) -> Json<IngestStatusResponse> {
    Json(IngestStatusResponse {
        status: "ready",
        accepted_total: state.accepted_total.load(Ordering::Relaxed),
        transport: "append-only-local-wal",
        acknowledgement_mode: if state.sync_writes { "fsync" } else { "flush" },
        durable_wal: state.sync_writes,
    })
}

async fn ingest_event(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<TelemetryInput>,
) -> Result<impl IntoResponse, ApiError> {
    let tenant_id = authenticate_tenant(&headers, &state.credentials)?;
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
            status: if state.sync_writes {
                "accepted_durable"
            } else {
                "accepted_buffered"
            },
            event_id: envelope.event_id.to_string(),
            signal: envelope.signal.to_string(),
            received_at,
        }),
    ))
}

fn authenticate_tenant<'a>(
    headers: &HeaderMap,
    credentials: &'a [IngestCredential],
) -> Result<&'a str, ApiError> {
    let value = headers
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or(ApiError::Unauthorized)?;
    let (scheme, token) = value.split_once(' ').ok_or(ApiError::Unauthorized)?;

    if !scheme.eq_ignore_ascii_case("bearer") || token.is_empty() {
        return Err(ApiError::Unauthorized);
    }

    credentials
        .iter()
        .find(|credential| constant_time_eq(token.as_bytes(), credential.token.as_bytes()))
        .map(|credential| credential.tenant_id.as_str())
        .ok_or(ApiError::Unauthorized)
}

fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }

    left.iter()
        .zip(right)
        .fold(0_u8, |difference, (left, right)| {
            difference | (left ^ right)
        })
        == 0
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
