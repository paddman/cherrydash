CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE IF NOT EXISTS tenants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    slug VARCHAR(64) NOT NULL UNIQUE,
    name VARCHAR(160) NOT NULL,
    status VARCHAR(24) NOT NULL DEFAULT 'active',
    settings JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (slug ~ '^[A-Za-z0-9._-]{2,64}$')
);

CREATE TABLE IF NOT EXISTS sites (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name VARCHAR(160) NOT NULL,
    site_type VARCHAR(32) NOT NULL DEFAULT 'datacenter',
    timezone VARCHAR(64) NOT NULL DEFAULT 'Asia/Bangkok',
    labels JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (tenant_id, name)
);

CREATE TABLE IF NOT EXISTS edge_collectors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    site_id UUID REFERENCES sites(id) ON DELETE SET NULL,
    edge_key VARCHAR(160) NOT NULL,
    display_name VARCHAR(160) NOT NULL,
    version VARCHAR(64),
    status VARCHAR(24) NOT NULL DEFAULT 'unknown',
    capabilities JSONB NOT NULL DEFAULT '[]'::jsonb,
    last_seen_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (tenant_id, edge_key)
);

CREATE TABLE IF NOT EXISTS hosts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    site_id UUID REFERENCES sites(id) ON DELETE SET NULL,
    edge_collector_id UUID REFERENCES edge_collectors(id) ON DELETE SET NULL,
    host_key VARCHAR(256) NOT NULL,
    display_name VARCHAR(256) NOT NULL,
    host_type VARCHAR(48) NOT NULL DEFAULT 'server',
    status VARCHAR(24) NOT NULL DEFAULT 'unknown',
    labels JSONB NOT NULL DEFAULT '{}'::jsonb,
    inventory JSONB NOT NULL DEFAULT '{}'::jsonb,
    first_seen_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_seen_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (tenant_id, host_key)
);

CREATE TABLE IF NOT EXISTS monitor_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
    template_key VARCHAR(160) NOT NULL,
    name VARCHAR(200) NOT NULL,
    version INTEGER NOT NULL DEFAULT 1,
    specification JSONB NOT NULL,
    built_in BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE NULLS NOT DISTINCT (tenant_id, template_key, version)
);

CREATE TABLE IF NOT EXISTS monitors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    host_id UUID REFERENCES hosts(id) ON DELETE CASCADE,
    template_id UUID REFERENCES monitor_templates(id) ON DELETE SET NULL,
    monitor_key VARCHAR(256) NOT NULL,
    monitor_type VARCHAR(64) NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT true,
    interval_seconds INTEGER NOT NULL DEFAULT 60 CHECK (interval_seconds > 0),
    timeout_seconds INTEGER NOT NULL DEFAULT 10 CHECK (timeout_seconds > 0),
    configuration JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (tenant_id, monitor_key)
);

CREATE TABLE IF NOT EXISTS dashboards (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    dashboard_key VARCHAR(160) NOT NULL,
    title VARCHAR(240) NOT NULL,
    folder VARCHAR(240),
    revision INTEGER NOT NULL DEFAULT 1,
    definition JSONB NOT NULL,
    created_by VARCHAR(256),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (tenant_id, dashboard_key)
);

CREATE TABLE IF NOT EXISTS alert_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    rule_key VARCHAR(160) NOT NULL,
    name VARCHAR(240) NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT true,
    severity VARCHAR(24) NOT NULL DEFAULT 'warning',
    expression TEXT NOT NULL,
    evaluation_interval_seconds INTEGER NOT NULL DEFAULT 60,
    pending_period_seconds INTEGER NOT NULL DEFAULT 0,
    labels JSONB NOT NULL DEFAULT '{}'::jsonb,
    annotations JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (tenant_id, rule_key)
);

CREATE TABLE IF NOT EXISTS incidents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    incident_number BIGINT GENERATED ALWAYS AS IDENTITY,
    title VARCHAR(320) NOT NULL,
    severity VARCHAR(24) NOT NULL,
    status VARCHAR(24) NOT NULL DEFAULT 'open',
    summary TEXT,
    evidence JSONB NOT NULL DEFAULT '[]'::jsonb,
    opened_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    acknowledged_at TIMESTAMPTZ,
    resolved_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (tenant_id, incident_number)
);

CREATE TABLE IF NOT EXISTS audit_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID REFERENCES tenants(id) ON DELETE SET NULL,
    actor_type VARCHAR(32) NOT NULL,
    actor_id VARCHAR(256) NOT NULL,
    action VARCHAR(160) NOT NULL,
    resource_type VARCHAR(96) NOT NULL,
    resource_id VARCHAR(256),
    request_id VARCHAR(96),
    before_state JSONB,
    after_state JSONB,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_hosts_tenant_status ON hosts (tenant_id, status);
CREATE INDEX IF NOT EXISTS idx_hosts_last_seen ON hosts (tenant_id, last_seen_at DESC);
CREATE INDEX IF NOT EXISTS idx_edges_last_seen ON edge_collectors (tenant_id, last_seen_at DESC);
CREATE INDEX IF NOT EXISTS idx_alert_rules_enabled ON alert_rules (tenant_id, enabled);
CREATE INDEX IF NOT EXISTS idx_incidents_status ON incidents (tenant_id, status, opened_at DESC);
CREATE INDEX IF NOT EXISTS idx_audit_tenant_time ON audit_events (tenant_id, occurred_at DESC);

INSERT INTO tenants (slug, name)
VALUES ('default', 'Default Workspace')
ON CONFLICT (slug) DO NOTHING;
