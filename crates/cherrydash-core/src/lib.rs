#![forbid(unsafe_code)]

pub mod telemetry;

pub use telemetry::{
    TelemetryEnvelope, TelemetryInput, TelemetrySignal, ValidationError, validate_source,
    validate_tenant_id,
};
