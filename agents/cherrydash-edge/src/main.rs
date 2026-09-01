// SPDX-License-Identifier: Apache-2.0
#![forbid(unsafe_code)]

use std::{collections::BTreeMap, time::Duration};

use anyhow::{Context, bail};
use cherrydash_core::{TelemetryInput, TelemetrySignal};
use chrono::Utc;
use clap::Parser;
use reqwest::Client;
use serde_json::{Value, json};
use tokio::time::{MissedTickBehavior, interval};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Clone, Parser)]
#[command(
    name = "cherrydash-edge",
    version,
    about = "CherryDash distributed edge collector"
)]
struct Settings {
    #[arg(
        long,
        env = "CHERRYDASH_INGEST_URL",
        default_value = "http://127.0.0.1:8081"
    )]
    ingest_url: String,

    #[arg(
        long,
        env = "CHERRYDASH_INGEST_TOKEN",
        default_value = "development-only-change-me"
    )]
    ingest_token: String,

    #[arg(long, env = "CHERRYDASH_EDGE_ID", default_value = "auto")]
    edge_id: String,

    #[arg(long, env = "CHERRYDASH_EDGE_INTERVAL_SECONDS", default_value_t = 15)]
    interval_seconds: u64,

    #[arg(long, env = "CHERRYDASH_EDGE_TIMEOUT_SECONDS", default_value_t = 10)]
    timeout_seconds: u64,

    #[arg(long, env = "CHERRYDASH_LOG_JSON", default_value_t = false)]
    log_json: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut settings = Settings::parse();
    init_tracing(settings.log_json);

    if settings.ingest_token.len() < 24 {
        bail!("CHERRYDASH_INGEST_TOKEN must contain at least 24 bytes");
    }

    let hostname = hostname();
    if settings.edge_id == "auto" {
        settings.edge_id = hostname.clone();
    }

    let endpoint = format!(
        "{}/api/v1/events",
        settings.ingest_url.trim_end_matches('/')
    );
    let client = Client::builder()
        .timeout(Duration::from_secs(settings.timeout_seconds.max(1)))
        .user_agent(format!("cherrydash-edge/{}", env!("CARGO_PKG_VERSION")))
        .build()
        .context("failed to create HTTP client")?;

    let mut ticker = interval(Duration::from_secs(settings.interval_seconds.max(1)));
    ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);
    let shutdown = shutdown_signal();
    tokio::pin!(shutdown);

    tracing::info!(
        edge_id = %settings.edge_id,
        endpoint = %endpoint,
        "CherryDash edge collector started"
    );

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                if let Err(error) = send_heartbeat(&client, &endpoint, &settings, &hostname).await {
                    tracing::warn!(%error, "edge heartbeat delivery failed; durable local queue is not implemented yet");
                }
            }
            _ = &mut shutdown => {
                tracing::info!("edge collector stopping");
                break;
            }
        }
    }

    Ok(())
}

async fn send_heartbeat(
    client: &Client,
    endpoint: &str,
    settings: &Settings,
    hostname: &str,
) -> anyhow::Result<()> {
    let mut attributes = BTreeMap::new();
    attributes.insert("collector.kind".to_owned(), "edge".to_owned());
    attributes.insert("host.name".to_owned(), hostname.to_owned());
    attributes.insert("os.type".to_owned(), std::env::consts::OS.to_owned());
    attributes.insert("host.arch".to_owned(), std::env::consts::ARCH.to_owned());

    let input = TelemetryInput {
        signal: TelemetrySignal::Heartbeat,
        source: format!("edge/{}", settings.edge_id),
        observed_at: Some(Utc::now()),
        attributes,
        body: host_snapshot(&settings.edge_id, hostname).await,
    };

    let response = client
        .post(endpoint)
        .bearer_auth(&settings.ingest_token)
        .json(&input)
        .send()
        .await
        .context("request to ingestion gateway failed")?;

    let status = response.status();
    if !status.is_success() {
        let response_body = response.text().await.unwrap_or_default();
        bail!("ingestion gateway returned {status}: {response_body}");
    }

    tracing::debug!(%status, "edge heartbeat accepted");
    Ok(())
}

async fn host_snapshot(edge_id: &str, hostname: &str) -> Value {
    let load_average = read_load_average().await;
    let memory = read_memory().await;
    let uptime_seconds = read_uptime_seconds().await;

    json!({
        "edgeId": edge_id,
        "hostname": hostname,
        "os": std::env::consts::OS,
        "architecture": std::env::consts::ARCH,
        "agentVersion": env!("CARGO_PKG_VERSION"),
        "loadAverage": load_average,
        "memory": memory,
        "uptimeSeconds": uptime_seconds,
    })
}

async fn read_load_average() -> Option<Vec<f64>> {
    let content = tokio::fs::read_to_string("/proc/loadavg").await.ok()?;
    let values = content
        .split_whitespace()
        .take(3)
        .filter_map(|value| value.parse::<f64>().ok())
        .collect::<Vec<_>>();

    (values.len() == 3).then_some(values)
}

async fn read_memory() -> Option<Value> {
    let content = tokio::fs::read_to_string("/proc/meminfo").await.ok()?;
    let mut total_kib = None;
    let mut available_kib = None;

    for line in content.lines() {
        if let Some(value) = line.strip_prefix("MemTotal:") {
            total_kib = first_number(value);
        } else if let Some(value) = line.strip_prefix("MemAvailable:") {
            available_kib = first_number(value);
        }
    }

    Some(json!({
        "totalBytes": total_kib.map(|value| value.saturating_mul(1024)),
        "availableBytes": available_kib.map(|value| value.saturating_mul(1024)),
    }))
}

async fn read_uptime_seconds() -> Option<u64> {
    let content = tokio::fs::read_to_string("/proc/uptime").await.ok()?;
    content
        .split_whitespace()
        .next()?
        .parse::<f64>()
        .ok()
        .map(|value| value.max(0.0) as u64)
}

fn first_number(value: &str) -> Option<u64> {
    value.split_whitespace().next()?.parse::<u64>().ok()
}

fn hostname() -> String {
    std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("COMPUTERNAME"))
        .unwrap_or_else(|_| "unknown-host".to_owned())
}

fn init_tracing(json: bool) {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("cherrydash=info"));

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
}
