# WAL and Delivery Semantics

The current NDJSON append file is a foundation mechanism, not the final durable queue. CherryDash must state precisely when data is accepted, durable, published and committed.

## Delivery states

```text
received
  ↓ validation + authentication
accepted_memory
  ↓ append
accepted_wal
  ↓ fsync durability boundary
accepted_durable
  ↓ publish to event fabric
published
  ↓ storage/idempotency commit
committed
```

API responses and metrics must never claim a later state than the system actually reached.

## WAL v2 segment format

```text
Segment header
  magic
  format_version
  writer_id
  created_at
  first_sequence

Record
  record_version
  sequence
  event_id
  tenant_shard
  payload_length
  payload_crc32c
  payload

Segment footer/index
  last_sequence
  record_count
  segment_checksum
```

Records are length-delimited and checksummed. Startup recovery scans the active segment, truncates only the final incomplete record and never skips corruption silently.

## Writer architecture

```text
HTTP/gRPC receivers
        ↓
bounded per-priority queue
        ↓
single WAL writer task
  batch append
  timed/size fsync
        ↓
checkpointed replay publisher
        ↓
event fabric
```

Receivers do not lock and flush one shared file per request. Queue saturation produces explicit back-pressure and `Retry-After`, not unbounded memory growth.

## Ordering and idempotency

- ordering is guaranteed only within a writer/partition sequence
- global ordering is not promised
- every record has a stable `event_id`
- publisher retries are at-least-once
- consumers and storage writers are idempotent by `event_id` or deterministic record key
- checkpoints advance only after publish acknowledgement
- poison records move to quarantine with reason and evidence

## Disk-pressure policy

Data classes:

```text
P0 audit, alert state, automation evidence    never evict silently
P1 inventory, topology, operational events    preserve; stop intake if necessary
P2 metrics                                     downsample/drop only by declared policy
P3 debug logs                                  evict oldest by tenant quota policy
```

Required metrics include queue depth, oldest age, disk bytes, fsync latency, replay lag, retries, duplicates, corruption and evictions.

## Edge store-and-forward

The edge uses the same principles with bounded local storage, boot ID, monotonic sequence and signed configuration version. WAN failure must not discard accepted records. Reconnect replay must tolerate duplicate delivery.

## Crash-recovery test matrix

- kill before append
- kill during record write
- kill after append before fsync
- kill after fsync before response
- disk full during append
- corrupt active and closed segment
- event fabric unavailable
- publisher restart after acknowledgement ambiguity
- duplicate replay into storage
- clock jump and system reboot

No production release is allowed until these tests are automated and repeatable.
