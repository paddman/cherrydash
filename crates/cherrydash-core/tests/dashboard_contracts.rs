use cherrydash_core::DashboardDefinition;
use serde_json::Value;

#[test]
fn universal_dashboard_example_deserializes_and_validates() {
    let raw = include_str!("../../../examples/dashboards/universal-operations.dashboard.json");
    let dashboard: DashboardDefinition =
        serde_json::from_str(raw).expect("example dashboard must deserialize");

    dashboard
        .validate()
        .expect("example dashboard must satisfy the Rust contract");
}

#[test]
fn public_dashboard_contract_files_are_valid_json() {
    let contracts = [
        (
            "dashboard schema",
            include_str!("../../../schemas/dashboard/v1/dashboard.schema.json"),
        ),
        (
            "renderer manifest schema",
            include_str!("../../../schemas/dashboard/v1/renderer-manifest.schema.json"),
        ),
        (
            "domain pack schema",
            include_str!("../../../schemas/domain-pack/v1/domain-pack.schema.json"),
        ),
    ];

    for (name, raw) in contracts {
        serde_json::from_str::<Value>(raw)
            .unwrap_or_else(|error| panic!("{name} must be valid JSON: {error}"));
    }
}
