CREATE DATABASE IF NOT EXISTS cherrydash;

CREATE TABLE IF NOT EXISTS cherrydash.telemetry_events
(
    schema_version UInt16,
    event_id String,
    tenant_id LowCardinality(String),
    signal LowCardinality(String),
    source String,
    observed_at DateTime64(3, 'UTC'),
    received_at DateTime64(3, 'UTC'),
    attributes Map(String, String),
    body String,
    received_date Date MATERIALIZED toDate(received_at),
    INDEX idx_source_token source TYPE tokenbf_v1(32768, 3, 0) GRANULARITY 4,
    INDEX idx_attributes_key mapKeys(attributes) TYPE bloom_filter(0.01) GRANULARITY 4
)
ENGINE = MergeTree
PARTITION BY (toYYYYMM(received_at), cityHash64(tenant_id) % 16)
ORDER BY (tenant_id, signal, source, received_at, event_id)
SETTINGS index_granularity = 8192;

CREATE TABLE IF NOT EXISTS cherrydash.metric_points
(
    tenant_id LowCardinality(String),
    metric_name String,
    metric_type LowCardinality(String),
    timestamp DateTime64(3, 'UTC'),
    value Float64,
    labels Map(String, String),
    resource_attributes Map(String, String),
    exemplar_trace_id String DEFAULT '',
    ingested_at DateTime64(3, 'UTC') DEFAULT now64(3),
    INDEX idx_metric_name metric_name TYPE bloom_filter(0.01) GRANULARITY 4,
    INDEX idx_labels_key mapKeys(labels) TYPE bloom_filter(0.01) GRANULARITY 4
)
ENGINE = MergeTree
PARTITION BY (toYYYYMM(timestamp), cityHash64(tenant_id) % 16)
ORDER BY (tenant_id, metric_name, timestamp, cityHash64(labels))
SETTINGS index_granularity = 8192;

CREATE TABLE IF NOT EXISTS cherrydash.edge_heartbeats
(
    tenant_id LowCardinality(String),
    edge_id String,
    hostname String,
    version LowCardinality(String),
    observed_at DateTime64(3, 'UTC'),
    payload String
)
ENGINE = ReplacingMergeTree(observed_at)
PARTITION BY toYYYYMM(observed_at)
ORDER BY (tenant_id, edge_id);
