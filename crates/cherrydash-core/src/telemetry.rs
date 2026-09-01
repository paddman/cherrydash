use std::{collections::BTreeMap, fmt};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use ulid::Ulid;

pub const TELEMETRY_SCHEMA_VERSION: u16 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TelemetrySignal {
    Metric,
    Log,
    Trace,
    Event,
    Inventory,
    Heartbeat,
}

impl fmt::Display for TelemetrySignal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Metric => "metric",
            Self::Log => "log",
            Self::Trace => "trace",
            Self::Event => "event",
            Self::Inventory => "inventory",
            Self::Heartbeat => "heartbeat",
        };
        formatter.write_str(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryInput {
    pub signal: TelemetrySignal,
    pub source: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub attributes: BTreeMap<String, String>,
    pub body: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryEnvelope {
    pub schema_version: u16,
    pub event_id: Ulid,
    pub tenant_id: String,
    pub signal: TelemetrySignal,
    pub source: String,
    pub observed_at: DateTime<Utc>,
    pub received_at: DateTime<Utc>,
    #[serde(default)]
    pub attributes: BTreeMap<String, String>,
    pub body: Value,
}

impl TelemetryEnvelope {
    pub fn from_input(
        tenant_id: impl Into<String>,
        input: TelemetryInput,
        received_at: DateTime<Utc>,
    ) -> Result<Self, ValidationError> {
        let tenant_id = tenant_id.into();
        validate_tenant_id(&tenant_id)?;
        validate_source(&input.source)?;

        Ok(Self {
            schema_version: TELEMETRY_SCHEMA_VERSION,
            event_id: Ulid::new(),
            tenant_id,
            signal: input.signal,
            source: input.source,
            observed_at: input.observed_at.unwrap_or(received_at),
            received_at,
            attributes: input.attributes,
            body: input.body,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ValidationError {
    #[error("tenant id must contain between 2 and 64 characters")]
    InvalidTenantLength,
    #[error("tenant id may only contain ASCII letters, digits, dot, dash, or underscore")]
    InvalidTenantCharacters,
    #[error("source must contain between 1 and 256 characters")]
    InvalidSourceLength,
    #[error("source contains a control character")]
    InvalidSourceCharacters,
}

pub fn validate_tenant_id(value: &str) -> Result<(), ValidationError> {
    if !(2..=64).contains(&value.len()) {
        return Err(ValidationError::InvalidTenantLength);
    }

    if !value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'_'))
    {
        return Err(ValidationError::InvalidTenantCharacters);
    }

    Ok(())
}

pub fn validate_source(value: &str) -> Result<(), ValidationError> {
    if value.is_empty() || value.len() > 256 {
        return Err(ValidationError::InvalidSourceLength);
    }

    if value.chars().any(char::is_control) {
        return Err(ValidationError::InvalidSourceCharacters);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tenant_id_accepts_safe_partition_keys() {
        assert!(validate_tenant_id("acme-th_01.prod").is_ok());
    }

    #[test]
    fn tenant_id_rejects_path_and_subject_injection() {
        assert_eq!(
            validate_tenant_id("../../root"),
            Err(ValidationError::InvalidTenantCharacters)
        );
        assert_eq!(
            validate_tenant_id("tenant.*"),
            Err(ValidationError::InvalidTenantCharacters)
        );
    }

    #[test]
    fn envelope_uses_receive_time_when_observed_time_is_missing() {
        let now = Utc::now();
        let input = TelemetryInput {
            signal: TelemetrySignal::Event,
            source: "test/source".to_owned(),
            observed_at: None,
            attributes: BTreeMap::new(),
            body: serde_json::json!({"ok": true}),
        };

        let envelope = TelemetryEnvelope::from_input("tenant-01", input, now)
            .expect("envelope should be valid");

        assert_eq!(envelope.observed_at, now);
        assert_eq!(envelope.received_at, now);
        assert_eq!(envelope.schema_version, TELEMETRY_SCHEMA_VERSION);
    }
}
