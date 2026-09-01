// SPDX-License-Identifier: AGPL-3.0-only
import { useEffect, useMemo, useState } from "react";

type ApiState = "connecting" | "online" | "offline";

type Overview = {
  health: string;
  hostsTotal: number;
  hostsProblem: number;
  activeAlerts: number;
  eventsPerSecond: number;
  edgeCollectorsOnline: number;
  signals: string[];
  capabilities: string[];
};

const fallbackOverview: Overview = {
  health: "unknown",
  hostsTotal: 0,
  hostsProblem: 0,
  activeAlerts: 0,
  eventsPerSecond: 0,
  edgeCollectorsOnline: 0,
  signals: ["metrics", "logs", "traces", "events", "inventory"],
  capabilities: [
    "distributed-monitoring",
    "unified-dashboards",
    "multi-tenant",
    "open-telemetry",
    "prometheus-compatible",
    "automation-with-approval",
  ],
};

const navigation = [
  ["Overview", "⌂"],
  ["Infrastructure", "▦"],
  ["Services", "◇"],
  ["Dashboards", "◫"],
  ["Explore", "⌁"],
  ["Alerts", "△"],
  ["Incidents", "◎"],
  ["SLO & SLA", "◷"],
  ["Automation", "↯"],
] as const;

const trendPoints = {
  hosts: "2,34 22,28 42,31 62,20 82,22 102,13 122,16 142,8",
  problems: "2,29 22,30 42,20 62,27 82,14 102,22 122,11 142,18",
  alerts: "2,31 22,23 42,28 62,15 82,19 102,10 122,12 142,6",
  events: "2,33 22,24 42,27 62,17 82,21 102,8 122,15 142,4",
};

export default function App() {
  const [overview, setOverview] = useState<Overview>(fallbackOverview);
  const [apiState, setApiState] = useState<ApiState>("connecting");
  const [updatedAt, setUpdatedAt] = useState<Date | null>(null);

  useEffect(() => {
    const controller = new AbortController();
    const apiBase = import.meta.env.VITE_API_BASE_URL ?? "";

    fetch(`${apiBase}/api/v1/overview`, { signal: controller.signal })
      .then(async (response) => {
        if (!response.ok) {
          throw new Error(`overview request returned ${response.status}`);
        }
        return (await response.json()) as Overview;
      })
      .then((payload) => {
        setOverview(payload);
        setApiState("online");
        setUpdatedAt(new Date());
      })
      .catch((error: unknown) => {
        if (error instanceof DOMException && error.name === "AbortError") {
          return;
        }
        setApiState("offline");
      });

    return () => controller.abort();
  }, []);

  const metrics = useMemo(
    () => [
      {
        label: "Monitored hosts",
        value: overview.hostsTotal,
        hint: `${overview.edgeCollectorsOnline} edge collectors online`,
        points: trendPoints.hosts,
      },
      {
        label: "Hosts with problems",
        value: overview.hostsProblem,
        hint: "Prioritized by service impact",
        points: trendPoints.problems,
      },
      {
        label: "Active alerts",
        value: overview.activeAlerts,
        hint: "Deduplicated and correlated",
        points: trendPoints.alerts,
      },
      {
        label: "Events / second",
        value: overview.eventsPerSecond,
        hint: "Across all telemetry signals",
        points: trendPoints.events,
      },
    ],
    [overview],
  );

  return (
    <div className="app-shell">
      <aside className="sidebar">
        <div className="brand">
          <div className="brand-mark" aria-hidden="true">
            <span />
          </div>
          <div>
            <strong>CherryDash</strong>
            <small>OBSERVABILITY FABRIC</small>
          </div>
        </div>

        <div className="workspace-switcher">
          <span className="workspace-avatar">NT</span>
          <div>
            <small>Workspace</small>
            <strong>Enterprise Lab</strong>
          </div>
          <span className="chevron">⌄</span>
        </div>

        <nav className="nav-list" aria-label="Primary navigation">
          {navigation.map(([label, icon], index) => (
            <button className={index === 0 ? "nav-item active" : "nav-item"} key={label}>
              <span className="nav-icon">{icon}</span>
              <span>{label}</span>
              {label === "Alerts" && overview.activeAlerts > 0 ? (
                <span className="nav-badge">{formatNumber(overview.activeAlerts)}</span>
              ) : null}
            </button>
          ))}
        </nav>

        <div className="sidebar-footer">
          <div className="platform-health">
            <span className={`status-dot ${apiState}`} />
            <div>
              <strong>{apiState === "online" ? "Platform healthy" : "Platform connecting"}</strong>
              <small>Control plane · {apiState}</small>
            </div>
          </div>
          <button className="settings-button">⚙ Settings</button>
        </div>
      </aside>

      <main className="main-area">
        <header className="topbar">
          <div className="search-box">
            <span>⌕</span>
            <input aria-label="Search" placeholder="Search hosts, services, alerts, dashboards…" />
            <kbd>⌘ K</kbd>
          </div>
          <div className="top-actions">
            <button className="icon-button" aria-label="Help">?</button>
            <button className="icon-button" aria-label="Notifications">◌</button>
            <button className="user-button">
              <span className="user-avatar">PM</span>
              <span>paddman</span>
              <span>⌄</span>
            </button>
          </div>
        </header>

        <section className="content">
          <div className="page-heading">
            <div>
              <p className="eyebrow">OPERATIONS OVERVIEW</p>
              <h1>Infrastructure command center</h1>
              <p>
                One operational view across infrastructure, applications, telemetry, alerts, and incidents.
              </p>
            </div>
            <div className="heading-actions">
              <button className="secondary-button">Last 24 hours⌄</button>
              <button className="primary-button">＋ Add monitor</button>
            </div>
          </div>

          <div className="health-strip">
            <div className="health-title">
              <span className={`large-status ${apiState}`} />
              <div>
                <strong>{apiState === "online" ? "All core services operational" : "Connecting to control plane"}</strong>
                <small>
                  {updatedAt ? `Last refreshed ${updatedAt.toLocaleTimeString()}` : "Waiting for live platform state"}
                </small>
              </div>
            </div>
            <div className="signal-list">
              {overview.signals.map((signal) => (
                <span key={signal}><i />{signal}</span>
              ))}
            </div>
          </div>

          <div className="metric-grid">
            {metrics.map((metric) => (
              <MetricCard key={metric.label} {...metric} />
            ))}
          </div>

          <div className="dashboard-grid">
            <section className="panel span-two">
              <PanelHeader
                title="Telemetry throughput"
                subtitle="Unified ingestion across every signal and site"
                action="Explore data"
              />
              <div className="chart-area">
                <div className="chart-y-axis">
                  <span>100k</span><span>75k</span><span>50k</span><span>25k</span><span>0</span>
                </div>
                <div className="line-chart">
                  <div className="grid-line line-1" />
                  <div className="grid-line line-2" />
                  <div className="grid-line line-3" />
                  <div className="grid-line line-4" />
                  <svg viewBox="0 0 760 220" preserveAspectRatio="none" role="img" aria-label="Telemetry trend placeholder">
                    <defs>
                      <linearGradient id="area" x1="0" x2="0" y1="0" y2="1">
                        <stop offset="0%" stopColor="#1688ff" stopOpacity="0.26" />
                        <stop offset="100%" stopColor="#1688ff" stopOpacity="0" />
                      </linearGradient>
                    </defs>
                    <path className="area-path" d="M0 176 C45 165 80 170 118 145 S190 126 228 137 S306 92 350 104 S420 60 465 81 S535 42 590 57 S680 18 760 34 L760 220 L0 220 Z" />
                    <path className="main-path" d="M0 176 C45 165 80 170 118 145 S190 126 228 137 S306 92 350 104 S420 60 465 81 S535 42 590 57 S680 18 760 34" />
                    <path className="secondary-path" d="M0 194 C70 181 105 193 156 166 S250 177 300 142 S390 154 447 118 S530 137 585 100 S690 112 760 72" />
                  </svg>
                  <div className="chart-x-axis">
                    <span>00:00</span><span>04:00</span><span>08:00</span><span>12:00</span><span>16:00</span><span>20:00</span><span>Now</span>
                  </div>
                </div>
              </div>
              <div className="chart-legend">
                <span><i className="legend-primary" /> Metrics & events</span>
                <span><i className="legend-secondary" /> Logs & traces</span>
                <strong>{formatNumber(overview.eventsPerSecond)} EPS now</strong>
              </div>
            </section>

            <section className="panel">
              <PanelHeader title="Operational risk" subtitle="Problems grouped by impact" action="View alerts" />
              <div className="risk-score">
                <div className="risk-ring"><strong>0</strong><small>/100</small></div>
                <div><strong>Low risk</strong><p>No correlated incidents are currently reported.</p></div>
              </div>
              <div className="severity-list">
                {[
                  ["Critical", 0], ["High", 0], ["Medium", 0], ["Warning", 0],
                ].map(([name, value]) => (
                  <div className="severity-row" key={name}>
                    <span><i className={`severity ${String(name).toLowerCase()}`} />{name}</span>
                    <strong>{value}</strong>
                  </div>
                ))}
              </div>
            </section>

            <section className="panel">
              <PanelHeader title="Collector fabric" subtitle="Distributed monitoring coverage" action="Manage edges" />
              <div className="collector-map">
                <div className="collector central"><span>CD</span><strong>Central</strong><small>Control plane</small></div>
                <div className="connector horizontal" />
                <div className="collector edge"><span>E1</span><strong>Edge</strong><small>Bangkok DC</small></div>
                <div className="collector edge second"><span>E2</span><strong>Edge</strong><small>Customer site</small></div>
              </div>
              <div className="collector-summary">
                <div><strong>{overview.edgeCollectorsOnline}</strong><span>Online</span></div>
                <div><strong>0</strong><span>Degraded</span></div>
                <div><strong>0</strong><span>Offline</span></div>
              </div>
            </section>

            <section className="panel span-two">
              <PanelHeader title="Recent operational activity" subtitle="Alerts, discoveries, configuration, and automation" action="View timeline" />
              <div className="empty-state">
                <div className="empty-icon">✓</div>
                <div>
                  <strong>No recent incidents</strong>
                  <p>Live events will appear here after collectors begin sending telemetry.</p>
                </div>
                <button className="secondary-button">Connect a collector</button>
              </div>
            </section>
          </div>
        </section>
      </main>
    </div>
  );
}

function MetricCard({
  label,
  value,
  hint,
  points,
}: {
  label: string;
  value: number;
  hint: string;
  points: string;
}) {
  return (
    <article className="metric-card">
      <div>
        <span className="metric-label">{label}</span>
        <strong className="metric-value">{formatNumber(value)}</strong>
        <small>{hint}</small>
      </div>
      <svg viewBox="0 0 144 40" aria-hidden="true">
        <polyline points={points} />
      </svg>
    </article>
  );
}

function PanelHeader({ title, subtitle, action }: { title: string; subtitle: string; action: string }) {
  return (
    <header className="panel-header">
      <div><h2>{title}</h2><p>{subtitle}</p></div>
      <button>{action} →</button>
    </header>
  );
}

function formatNumber(value: number): string {
  return new Intl.NumberFormat("en-US", { notation: value >= 100_000 ? "compact" : "standard" }).format(value);
}
