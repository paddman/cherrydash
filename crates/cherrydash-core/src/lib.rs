#![forbid(unsafe_code)]

pub mod dashboard;
pub mod telemetry;

pub use dashboard::{
    DASHBOARD_SCHEMA_VERSION, DashboardDefinition, DashboardLayout, DashboardQuery,
    DashboardSettings, DashboardValidationError, DashboardVariable, DataSourceBinding,
    InteractionDefinition, PanelAccessibility, PanelDefinition, PanelPlacement,
    TransformDefinition, VariableOption,
};
pub use telemetry::{
    TelemetryEnvelope, TelemetryInput, TelemetrySignal, ValidationError, validate_source,
    validate_tenant_id,
};
