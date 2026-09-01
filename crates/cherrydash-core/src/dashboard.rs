use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

pub const DASHBOARD_SCHEMA_VERSION: u16 = 1;

const MAX_DATA_SOURCES: usize = 256;
const MAX_VARIABLES: usize = 256;
const MAX_LAYOUTS: usize = 32;
const MAX_PANELS: usize = 2_000;
const MAX_PANEL_QUERIES: usize = 64;
const MAX_LAYOUT_COLUMNS: u16 = 64;
const MAX_ROW_HEIGHT_PX: u16 = 512;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardDefinition {
    pub schema_version: u16,
    pub key: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default = "default_locale")]
    pub locale: String,
    #[serde(default = "default_timezone")]
    pub timezone: String,
    #[serde(default)]
    pub domain_profiles: Vec<String>,
    #[serde(default)]
    pub data_sources: Vec<DataSourceBinding>,
    #[serde(default)]
    pub variables: Vec<DashboardVariable>,
    #[serde(default)]
    pub layouts: Vec<DashboardLayout>,
    #[serde(default)]
    pub panels: Vec<PanelDefinition>,
    #[serde(default)]
    pub settings: DashboardSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceBinding {
    pub id: String,
    pub adapter: String,
    #[serde(default)]
    pub configuration: Value,
    #[serde(default)]
    pub secret_refs: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardVariable {
    pub name: String,
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<DashboardQuery>,
    #[serde(default)]
    pub default_value: Value,
    #[serde(default)]
    pub allow_multiple: bool,
    #[serde(default)]
    pub include_all: bool,
    #[serde(default)]
    pub options: Vec<VariableOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariableOption {
    pub label: String,
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardLayout {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_width_px: Option<u32>,
    pub columns: u16,
    pub row_height_px: u16,
    #[serde(default)]
    pub dense: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PanelDefinition {
    pub id: String,
    pub title: String,
    pub renderer: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub queries: Vec<DashboardQuery>,
    #[serde(default)]
    pub placements: Vec<PanelPlacement>,
    #[serde(default)]
    pub transformations: Vec<TransformDefinition>,
    #[serde(default)]
    pub interactions: Vec<InteractionDefinition>,
    #[serde(default)]
    pub options: Value,
    #[serde(default)]
    pub accessibility: PanelAccessibility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PanelPlacement {
    pub layout: String,
    pub x: u16,
    pub y: u32,
    pub width: u16,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardQuery {
    pub id: String,
    pub data_source_ref: String,
    pub query_type: String,
    #[serde(default)]
    pub expression: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_shape: Option<String>,
    #[serde(default)]
    pub options: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformDefinition {
    pub kind: String,
    #[serde(default)]
    pub configuration: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractionDefinition {
    pub event: String,
    pub action: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default)]
    pub configuration: Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PanelAccessibility {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub table_fallback: bool,
    #[serde(default)]
    pub color_independent: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_time_range: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_interval_seconds: Option<u32>,
    #[serde(default)]
    pub live_mode: bool,
    #[serde(default)]
    pub display_modes: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_query_concurrency: Option<u16>,
}

impl DashboardDefinition {
    pub fn validate(&self) -> Result<(), DashboardValidationError> {
        if self.schema_version != DASHBOARD_SCHEMA_VERSION {
            return Err(DashboardValidationError::UnsupportedSchemaVersion(
                self.schema_version,
            ));
        }

        validate_identifier("dashboard key", &self.key)?;
        validate_text("dashboard title", &self.title, 240)?;
        validate_identifier("locale", &self.locale)?;
        validate_timezone(&self.timezone)?;

        enforce_limit("data sources", self.data_sources.len(), MAX_DATA_SOURCES)?;
        enforce_limit("variables", self.variables.len(), MAX_VARIABLES)?;
        enforce_limit("layouts", self.layouts.len(), MAX_LAYOUTS)?;
        enforce_limit("panels", self.panels.len(), MAX_PANELS)?;

        let mut domain_profiles = BTreeSet::new();
        for profile in &self.domain_profiles {
            validate_identifier("domain profile", profile)?;
            insert_unique(&mut domain_profiles, "domain profile", profile)?;
        }

        let mut data_sources = BTreeSet::new();
        for data_source in &self.data_sources {
            validate_identifier("data source id", &data_source.id)?;
            validate_identifier("data source adapter", &data_source.adapter)?;
            insert_unique(&mut data_sources, "data source id", &data_source.id)?;
            validate_object(
                "configuration",
                &format!("data source {}", data_source.id),
                &data_source.configuration,
            )?;
            reject_inline_secrets(
                &format!("data source {}", data_source.id),
                &data_source.configuration,
            )?;

            for (name, reference) in &data_source.secret_refs {
                validate_identifier("secret binding name", name)?;
                validate_secret_reference(reference)?;
            }
        }

        let mut layouts = BTreeMap::new();
        for layout in &self.layouts {
            validate_identifier("layout key", &layout.key)?;
            if layouts
                .insert(layout.key.as_str(), layout.columns)
                .is_some()
            {
                return Err(DashboardValidationError::Duplicate {
                    kind: "layout key",
                    value: layout.key.clone(),
                });
            }
            if layout.columns == 0 || layout.columns > MAX_LAYOUT_COLUMNS {
                return Err(DashboardValidationError::InvalidLayout {
                    layout: layout.key.clone(),
                    message: format!("columns must be between 1 and {MAX_LAYOUT_COLUMNS}"),
                });
            }
            if layout.row_height_px == 0 || layout.row_height_px > MAX_ROW_HEIGHT_PX {
                return Err(DashboardValidationError::InvalidLayout {
                    layout: layout.key.clone(),
                    message: format!("rowHeightPx must be between 1 and {MAX_ROW_HEIGHT_PX}"),
                });
            }
        }

        let mut variables = BTreeSet::new();
        for variable in &self.variables {
            validate_identifier("variable name", &variable.name)?;
            validate_identifier("variable kind", &variable.kind)?;
            insert_unique(&mut variables, "variable name", &variable.name)?;
            if let Some(query) = &variable.query {
                validate_query(query, &data_sources, &format!("variable {}", variable.name))?;
            }
        }

        let mut panels = BTreeSet::new();
        for panel in &self.panels {
            validate_identifier("panel id", &panel.id)?;
            validate_text("panel title", &panel.title, 240)?;
            validate_identifier("panel renderer", &panel.renderer)?;
            insert_unique(&mut panels, "panel id", &panel.id)?;
            enforce_limit("panel queries", panel.queries.len(), MAX_PANEL_QUERIES)?;
            validate_object("options", &format!("panel {}", panel.id), &panel.options)?;
            reject_inline_secrets(&format!("panel {}", panel.id), &panel.options)?;

            let mut query_ids = BTreeSet::new();
            for query in &panel.queries {
                insert_unique(&mut query_ids, "query id", &query.id)?;
                validate_query(query, &data_sources, &format!("panel {}", panel.id))?;
            }

            let mut placement_layouts = BTreeSet::new();
            for placement in &panel.placements {
                insert_unique(
                    &mut placement_layouts,
                    "panel placement layout",
                    &placement.layout,
                )?;
                let columns = layouts.get(placement.layout.as_str()).ok_or_else(|| {
                    DashboardValidationError::UnknownLayout {
                        panel_id: panel.id.clone(),
                        layout: placement.layout.clone(),
                    }
                })?;
                validate_placement(panel, placement, *columns)?;
            }

            for transform in &panel.transformations {
                validate_identifier("transformation kind", &transform.kind)?;
                validate_object(
                    "configuration",
                    &format!("transformation {} in panel {}", transform.kind, panel.id),
                    &transform.configuration,
                )?;
                reject_inline_secrets(
                    &format!("transformation {} in panel {}", transform.kind, panel.id),
                    &transform.configuration,
                )?;
            }

            for interaction in &panel.interactions {
                validate_identifier("interaction event", &interaction.event)?;
                validate_identifier("interaction action", &interaction.action)?;
                validate_object(
                    "configuration",
                    &format!("interaction {} in panel {}", interaction.action, panel.id),
                    &interaction.configuration,
                )?;
                reject_inline_secrets(
                    &format!("interaction {} in panel {}", interaction.action, panel.id),
                    &interaction.configuration,
                )?;
            }
        }

        for mode in &self.settings.display_modes {
            validate_identifier("display mode", mode)?;
        }
        if self.settings.refresh_interval_seconds == Some(0) {
            return Err(DashboardValidationError::InvalidSetting {
                setting: "refreshIntervalSeconds",
                message: "must be greater than zero when configured".to_owned(),
            });
        }
        if self.settings.max_query_concurrency == Some(0) {
            return Err(DashboardValidationError::InvalidSetting {
                setting: "maxQueryConcurrency",
                message: "must be greater than zero when configured".to_owned(),
            });
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DashboardValidationError {
    #[error("unsupported dashboard schema version {0}")]
    UnsupportedSchemaVersion(u16),
    #[error("{field} is invalid: {value}")]
    InvalidIdentifier { field: &'static str, value: String },
    #[error("{field} must contain between 1 and {max} visible characters")]
    InvalidText { field: &'static str, max: usize },
    #[error("{kind} exceeds the limit of {limit}")]
    TooMany { kind: &'static str, limit: usize },
    #[error("duplicate {kind}: {value}")]
    Duplicate { kind: &'static str, value: String },
    #[error("timezone is invalid: {0}")]
    InvalidTimezone(String),
    #[error("secret reference is invalid: {0}")]
    InvalidSecretReference(String),
    #[error("layout {layout}: {message}")]
    InvalidLayout { layout: String, message: String },
    #[error("panel {panel_id} references unknown layout {layout}")]
    UnknownLayout { panel_id: String, layout: String },
    #[error("panel {panel_id} placement in {layout}: {message}")]
    InvalidPlacement {
        panel_id: String,
        layout: String,
        message: String,
    },
    #[error("{owner} references unknown data source {data_source}")]
    UnknownDataSource { owner: String, data_source: String },
    #[error("{field} for {owner} must be a JSON object or null")]
    ExpectedObject { field: &'static str, owner: String },
    #[error("{owner} contains inline secret-like field {key}; use secretRefs")]
    InlineSecret { owner: String, key: String },
    #[error("invalid setting {setting}: {message}")]
    InvalidSetting {
        setting: &'static str,
        message: String,
    },
}

fn default_locale() -> String {
    "en-US".to_owned()
}

fn default_timezone() -> String {
    "UTC".to_owned()
}

fn enforce_limit(
    kind: &'static str,
    actual: usize,
    limit: usize,
) -> Result<(), DashboardValidationError> {
    if actual > limit {
        return Err(DashboardValidationError::TooMany { kind, limit });
    }
    Ok(())
}

fn insert_unique<'a>(
    values: &mut BTreeSet<&'a str>,
    kind: &'static str,
    value: &'a str,
) -> Result<(), DashboardValidationError> {
    if !values.insert(value) {
        return Err(DashboardValidationError::Duplicate {
            kind,
            value: value.to_owned(),
        });
    }
    Ok(())
}

fn validate_identifier(field: &'static str, value: &str) -> Result<(), DashboardValidationError> {
    let valid_length = (1..=128).contains(&value.len());
    let valid_characters = value.bytes().all(|byte| {
        byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'_' | b'/' | b':' | b'@')
    });

    if !valid_length || !valid_characters || value.contains("..") {
        return Err(DashboardValidationError::InvalidIdentifier {
            field,
            value: value.to_owned(),
        });
    }
    Ok(())
}

fn validate_text(
    field: &'static str,
    value: &str,
    max: usize,
) -> Result<(), DashboardValidationError> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.chars().count() > max || trimmed.chars().any(char::is_control)
    {
        return Err(DashboardValidationError::InvalidText { field, max });
    }
    Ok(())
}

fn validate_timezone(value: &str) -> Result<(), DashboardValidationError> {
    let valid = value == "UTC"
        || value == "browser"
        || (value.contains('/')
            && value.len() <= 128
            && value.bytes().all(|byte| {
                byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'_' | b'-' | b'+')
            }));

    if !valid {
        return Err(DashboardValidationError::InvalidTimezone(value.to_owned()));
    }
    Ok(())
}

fn validate_secret_reference(value: &str) -> Result<(), DashboardValidationError> {
    let valid = (3..=512).contains(&value.len())
        && !value.chars().any(char::is_control)
        && !value.chars().any(char::is_whitespace);
    if !valid {
        return Err(DashboardValidationError::InvalidSecretReference(
            value.to_owned(),
        ));
    }
    Ok(())
}

fn validate_query(
    query: &DashboardQuery,
    data_sources: &BTreeSet<&str>,
    owner: &str,
) -> Result<(), DashboardValidationError> {
    validate_identifier("query id", &query.id)?;
    validate_identifier("query type", &query.query_type)?;
    validate_identifier("query data source", &query.data_source_ref)?;
    if !data_sources.contains(query.data_source_ref.as_str()) {
        return Err(DashboardValidationError::UnknownDataSource {
            owner: owner.to_owned(),
            data_source: query.data_source_ref.clone(),
        });
    }
    if let Some(shape) = &query.expected_shape {
        validate_identifier("expected data shape", shape)?;
    }
    validate_object(
        "options",
        &format!("query {} in {owner}", query.id),
        &query.options,
    )?;
    reject_inline_secrets(&format!("query {} in {owner}", query.id), &query.expression)?;
    reject_inline_secrets(&format!("query {} in {owner}", query.id), &query.options)?;
    Ok(())
}

fn validate_placement(
    panel: &PanelDefinition,
    placement: &PanelPlacement,
    columns: u16,
) -> Result<(), DashboardValidationError> {
    if placement.width == 0 || placement.height == 0 {
        return Err(DashboardValidationError::InvalidPlacement {
            panel_id: panel.id.clone(),
            layout: placement.layout.clone(),
            message: "width and height must be greater than zero".to_owned(),
        });
    }

    let end_column = placement.x.checked_add(placement.width).ok_or_else(|| {
        DashboardValidationError::InvalidPlacement {
            panel_id: panel.id.clone(),
            layout: placement.layout.clone(),
            message: "column range overflowed".to_owned(),
        }
    })?;
    if end_column > columns {
        return Err(DashboardValidationError::InvalidPlacement {
            panel_id: panel.id.clone(),
            layout: placement.layout.clone(),
            message: format!("x + width must not exceed {columns} columns"),
        });
    }
    Ok(())
}

fn validate_object(
    field: &'static str,
    owner: &str,
    value: &Value,
) -> Result<(), DashboardValidationError> {
    if !value.is_null() && !value.is_object() {
        return Err(DashboardValidationError::ExpectedObject {
            field,
            owner: owner.to_owned(),
        });
    }
    Ok(())
}

fn reject_inline_secrets(owner: &str, value: &Value) -> Result<(), DashboardValidationError> {
    if let Some(key) = find_sensitive_key(value) {
        return Err(DashboardValidationError::InlineSecret {
            owner: owner.to_owned(),
            key,
        });
    }
    Ok(())
}

fn find_sensitive_key(value: &Value) -> Option<String> {
    match value {
        Value::Object(object) => {
            for (key, nested) in object {
                if is_sensitive_key(key) {
                    return Some(key.clone());
                }
                if let Some(found) = find_sensitive_key(nested) {
                    return Some(found);
                }
            }
            None
        }
        Value::Array(values) => values.iter().find_map(find_sensitive_key),
        _ => None,
    }
}

fn is_sensitive_key(value: &str) -> bool {
    let normalized = value
        .bytes()
        .filter(|byte| byte.is_ascii_alphanumeric())
        .map(|byte| byte.to_ascii_lowercase() as char)
        .collect::<String>();

    matches!(
        normalized.as_str(),
        "password"
            | "passwd"
            | "token"
            | "apitoken"
            | "apikey"
            | "secret"
            | "clientsecret"
            | "credential"
            | "credentials"
            | "privatekey"
            | "accesskey"
            | "accesskeyid"
            | "secretaccesskey"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn accepts_industry_neutral_dashboard_with_custom_renderer() {
        let dashboard = sample_dashboard();
        assert!(dashboard.validate().is_ok());
    }

    #[test]
    fn rejects_duplicate_panel_ids() {
        let mut dashboard = sample_dashboard();
        dashboard.panels.push(dashboard.panels[0].clone());

        assert_eq!(
            dashboard.validate(),
            Err(DashboardValidationError::Duplicate {
                kind: "panel id",
                value: "plant-overview".to_owned(),
            })
        );
    }

    #[test]
    fn rejects_unknown_data_source() {
        let mut dashboard = sample_dashboard();
        dashboard.panels[0].queries[0].data_source_ref = "missing-source".to_owned();

        assert_eq!(
            dashboard.validate(),
            Err(DashboardValidationError::UnknownDataSource {
                owner: "panel plant-overview".to_owned(),
                data_source: "missing-source".to_owned(),
            })
        );
    }

    #[test]
    fn rejects_inline_credentials() {
        let mut dashboard = sample_dashboard();
        dashboard.data_sources[0].configuration = json!({
            "endpoint": "https://example.invalid",
            "password": "do-not-store-this"
        });

        assert_eq!(
            dashboard.validate(),
            Err(DashboardValidationError::InlineSecret {
                owner: "data source operations".to_owned(),
                key: "password".to_owned(),
            })
        );
    }

    #[test]
    fn rejects_panel_outside_layout_columns() {
        let mut dashboard = sample_dashboard();
        dashboard.panels[0].placements[0].width = 13;

        assert_eq!(
            dashboard.validate(),
            Err(DashboardValidationError::InvalidPlacement {
                panel_id: "plant-overview".to_owned(),
                layout: "desktop".to_owned(),
                message: "x + width must not exceed 12 columns".to_owned(),
            })
        );
    }

    fn sample_dashboard() -> DashboardDefinition {
        DashboardDefinition {
            schema_version: DASHBOARD_SCHEMA_VERSION,
            key: "global.operations".to_owned(),
            title: "Global Operations".to_owned(),
            description: Some("Industry-neutral dashboard contract".to_owned()),
            tags: vec!["operations".to_owned()],
            locale: "th-TH".to_owned(),
            timezone: "Asia/Bangkok".to_owned(),
            domain_profiles: vec![
                "manufacturing.operations".to_owned(),
                "healthcare.facilities".to_owned(),
            ],
            data_sources: vec![DataSourceBinding {
                id: "operations".to_owned(),
                adapter: "cherrydash.native.telemetry/v1".to_owned(),
                configuration: json!({"tenantScoped": true}),
                secret_refs: BTreeMap::new(),
            }],
            variables: vec![],
            layouts: vec![DashboardLayout {
                key: "desktop".to_owned(),
                min_width_px: Some(1_024),
                columns: 12,
                row_height_px: 32,
                dense: true,
            }],
            panels: vec![PanelDefinition {
                id: "plant-overview".to_owned(),
                title: "Operations Overview".to_owned(),
                renderer: "cherrydash.visualization.process_mimic/v1".to_owned(),
                description: None,
                queries: vec![DashboardQuery {
                    id: "status".to_owned(),
                    data_source_ref: "operations".to_owned(),
                    query_type: "resource_status".to_owned(),
                    expression: json!({"resourceType": "site"}),
                    expected_shape: Some("topology".to_owned()),
                    options: Value::Null,
                }],
                placements: vec![PanelPlacement {
                    layout: "desktop".to_owned(),
                    x: 0,
                    y: 0,
                    width: 12,
                    height: 8,
                }],
                transformations: vec![],
                interactions: vec![],
                options: json!({"showLegend": true}),
                accessibility: PanelAccessibility {
                    label: Some("Operations topology".to_owned()),
                    description: None,
                    table_fallback: true,
                    color_independent: true,
                },
            }],
            settings: DashboardSettings {
                default_time_range: Some("PT24H".to_owned()),
                refresh_interval_seconds: Some(30),
                live_mode: true,
                display_modes: vec!["desktop".to_owned(), "wallboard".to_owned()],
                max_query_concurrency: Some(8),
            },
        }
    }
}
