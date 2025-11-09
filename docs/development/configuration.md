# Configuration Guide

This guide explains how to configure Phenix-DB for different deployment scenarios.

## Configuration Methods

Phenix-DB supports multiple configuration methods (in order of precedence):

1. **Environment Variables** (highest priority)
2. **Configuration File** (TOML)
3. **Default Values** (lowest priority)

## Configuration File

### Location

The configuration file can be specified via:

```bash
# Environment variable
export PHENIX_CONFIG=/path/to/phenix-db.toml

# Command line argument
phenix-server --config /path/to/phenix-db.toml
```

Default locations (searched in order):
1. `./phenix-db.toml`
2. `~/.config/phenix-db/config.toml`
3. `/etc/phenix-db/config.toml`

### Example Configuration

```toml
# phenix-db.toml

[polynomial_index]
# Polynomial degree for embeddings (default: 5)
# Higher degree = more accurate but slower
# Range: 3-20
degree = 5

# Branching factor for tree structure (default: 16)
# Higher = flatter tree, more memory
# Range: 8-64
branching_factor = 16

# Precision tolerance for polynomial evaluation (default: 0.00001)
# Lower = more accurate but slower
precision_tolerance = 0.00001

[probabilistic_graph]
# Learning rate for weight updates (default: 0.1)
# Higher = faster adaptation but less stable
# Range: 0.01-0.5
learning_rate = 0.1

# Pruning threshold for low-probability edges (default: 0.01)
# Higher = more aggressive pruning
# Range: 0.001-0.1
pruning_threshold = 0.01

# Co-access time window in milliseconds (default: 100)
# Entities accessed within this window are considered co-accessed
co_access_window_ms = 100

# Normalization interval in seconds (default: 10)
# How often to normalize probability distributions
normalization_interval_sec = 10

[bellman_optimizer]
# Observation window size for access patterns (default: 1000)
# Number of accesses to observe before optimization
observation_window = 1000

# Restructure threshold multiplier (default: 1.5)
# Trigger restructuring when cost exceeds optimal * threshold
restructure_threshold = 1.5

# Update interval in seconds (default: 10)
# How often to recompute optimal paths
update_interval_sec = 10

# Discount factor for dynamic programming (default: 0.95)
# Weight for future costs vs immediate costs
# Range: 0.0-1.0
discount_factor = 0.95

[compression]
# Target compression ratio (default: 0.8 = 80%)
# Actual ratio may vary based on data
# Range: 0.5-0.95
target_ratio = 0.8

# Maximum decompression time in milliseconds (default: 5)
# Fallback to uncompressed if exceeded
max_decompression_ms = 5

# Minimum pattern frequency for dictionary (default: 100)
# Patterns appearing less frequently won't be in dictionary
min_pattern_frequency = 100

# Ramanujan series degree (default: 10)
# Higher = better compression but slower
# Range: 5-20
ramanujan_series_degree = 10

[learning]
# Convergence threshold for learning algorithms (default: 0.001)
# Stop learning when improvement < threshold
convergence_threshold = 0.001

# Sample window size (default: 1000)
# Number of samples for learning
sample_window = 1000

# Minimum prediction accuracy (default: 0.75)
# Disable learning if accuracy drops below this
# Range: 0.5-0.95
min_accuracy = 0.75

# Maximum CPU overhead for learning (default: 0.05 = 5%)
# Throttle learning if overhead exceeds this
# Range: 0.01-0.2
max_cpu_overhead = 0.05

[tiering]
# Hot tier promotion threshold in accesses/hour (default: 100.0)
# Promote to hot tier if access frequency exceeds this
hot_promotion_threshold = 100.0

# Warm tier promotion threshold in accesses/hour (default: 10.0)
warm_promotion_threshold = 10.0

# Hot tier demotion threshold in accesses/hour (default: 1.0)
# Demote from hot tier if access frequency drops below this
hot_demotion_threshold = 1.0

# Warm tier demotion threshold in accesses/hour (default: 0.1)
warm_demotion_threshold = 0.1

# Hot tier demotion period in hours (default: 24)
# Wait this long before demoting from hot tier
hot_demotion_period_hours = 24

# Warm tier demotion period in days (default: 7)
warm_demotion_period_days = 7

[distributed]
# Minimum number of replicas (default: 3)
# For fault tolerance
# Range: 1-10
min_replicas = 3

# Maximum number of replicas (default: 10)
max_replicas = 10

# Awareness percentage of global state (default: 0.1 = 10%)
# Each node knows about this fraction of other nodes
# Range: 0.05-0.5
awareness_percentage = 0.1

# Consensus timeout in seconds (default: 5)
# Fallback to local decision if consensus not reached
consensus_timeout_sec = 5

# Heartbeat interval in milliseconds (default: 500)
# How often nodes send heartbeats
heartbeat_interval_ms = 500

[server]
# gRPC server address
grpc_address = "0.0.0.0:50051"

# REST API address
rest_address = "0.0.0.0:8080"

# Metrics endpoint address (Prometheus)
metrics_address = "0.0.0.0:9090"

# Maximum concurrent connections
max_connections = 10000

# Request timeout in seconds
request_timeout_sec = 30

[storage]
# Hot tier storage path (RAM/NVMe)
hot_tier_path = "/data/hot"

# Warm tier storage path (NVMe/SSD)
warm_tier_path = "/data/warm"

# Cold tier storage path or object storage URL
# Local: /data/cold
# S3: s3://bucket-name/prefix
# GCS: gs://bucket-name/prefix
cold_tier_path = "/data/cold"

# WAL directory
wal_path = "/data/wal"

# Maximum hot tier size in GB
hot_tier_max_size_gb = 100

# Maximum warm tier size in GB
warm_tier_max_size_gb = 1000

[security]
# Enable TLS for network communication
enable_tls = true

# TLS certificate path
tls_cert_path = "/etc/phenix-db/certs/server.crt"

# TLS key path
tls_key_path = "/etc/phenix-db/certs/server.key"

# Enable mTLS (mutual TLS) for node-to-node communication
enable_mtls = true

# Enable encryption at rest
enable_encryption_at_rest = true

# Encryption algorithm (aes-256-gcm, chacha20-poly1305)
encryption_algorithm = "aes-256-gcm"

# KMS endpoint (if using external KMS)
# Leave empty to use local key management
# kms_endpoint = "https://kms.example.com"

# Enable audit logging
enable_audit_logging = true

# Audit log path
audit_log_path = "/var/log/phenix-db/audit.log"

[observability]
# Log level (trace, debug, info, warn, error)
log_level = "info"

# Enable JSON logging (for structured log aggregation)
json_logging = true

# Log file path (empty = stdout)
log_file_path = ""

# OpenTelemetry endpoint for distributed tracing
# Leave empty to disable
# otel_endpoint = "http://localhost:4317"

# Enable Prometheus metrics
enable_metrics = true

# Metrics collection interval in seconds
metrics_interval_sec = 10

# Enable detailed mathematical metrics
# (polynomial evaluation times, compression ratios, etc.)
enable_mathematical_metrics = true
```

## Environment Variables

All configuration options can be overridden with environment variables using the prefix `PHENIX_`:

```bash
# Polynomial index configuration
export PHENIX_POLYNOMIAL_INDEX_DEGREE=7
export PHENIX_POLYNOMIAL_INDEX_BRANCHING_FACTOR=32

# Server configuration
export PHENIX_SERVER_GRPC_ADDRESS="0.0.0.0:50051"
export PHENIX_SERVER_REST_ADDRESS="0.0.0.0:8080"

# Storage paths
export PHENIX_STORAGE_HOT_TIER_PATH="/mnt/nvme/hot"
export PHENIX_STORAGE_WARM_TIER_PATH="/mnt/ssd/warm"
export PHENIX_STORAGE_COLD_TIER_PATH="s3://my-bucket/cold"

# Security
export PHENIX_SECURITY_ENABLE_TLS=true
export PHENIX_SECURITY_TLS_CERT_PATH="/etc/certs/server.crt"

# Observability
export PHENIX_OBSERVABILITY_LOG_LEVEL=debug
export PHENIX_OBSERVABILITY_OTEL_ENDPOINT="http://jaeger:4317"
```

## Configuration Profiles

### Development Profile

```toml
# phenix-db-dev.toml
[polynomial_index]
degree = 3  # Lower for faster iteration
branching_factor = 8

[learning]
max_cpu_overhead = 0.1  # Allow more overhead in dev

[observability]
log_level = "debug"
enable_mathematical_metrics = true

[storage]
hot_tier_path = "./data/hot"
warm_tier_path = "./data/warm"
cold_tier_path = "./data/cold"

[security]
enable_tls = false  # Disable for local development
enable_encryption_at_rest = false
```

### Production Profile

```toml
# phenix-db-prod.toml
[polynomial_index]
degree = 5
branching_factor = 16

[learning]
max_cpu_overhead = 0.05  # Strict overhead limit

[observability]
log_level = "info"
json_logging = true
otel_endpoint = "http://jaeger:4317"

[storage]
hot_tier_path = "/mnt/nvme/hot"
warm_tier_path = "/mnt/ssd/warm"
cold_tier_path = "s3://prod-bucket/cold"

[security]
enable_tls = true
enable_mtls = true
enable_encryption_at_rest = true
enable_audit_logging = true

[distributed]
min_replicas = 3
max_replicas = 10
```

### High-Performance Profile

```toml
# phenix-db-perf.toml
[polynomial_index]
degree = 7  # Higher accuracy
branching_factor = 32  # Flatter tree

[compression]
target_ratio = 0.9  # Aggressive compression
ramanujan_series_degree = 15

[tiering]
hot_promotion_threshold = 50.0  # More aggressive promotion
hot_demotion_period_hours = 48  # Keep in hot longer

[learning]
convergence_threshold = 0.0001  # More precise learning
sample_window = 5000  # Larger sample size
```

## Kubernetes ConfigMap

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: phenix-db-config
  namespace: phenix-db
data:
  phenix-db.toml: |
    [polynomial_index]
    degree = 5
    branching_factor = 16
    
    [server]
    grpc_address = "0.0.0.0:50051"
    rest_address = "0.0.0.0:8080"
    metrics_address = "0.0.0.0:9090"
    
    [storage]
    hot_tier_path = "/data/hot"
    warm_tier_path = "/data/warm"
    cold_tier_path = "s3://prod-bucket/cold"
    
    [security]
    enable_tls = true
    tls_cert_path = "/etc/certs/tls.crt"
    tls_key_path = "/etc/certs/tls.key"
    
    [observability]
    log_level = "info"
    json_logging = true
    otel_endpoint = "http://jaeger-collector:4317"
```

## Docker Environment

```dockerfile
# Dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/phenix-server /usr/local/bin/
COPY phenix-db.toml /etc/phenix-db/config.toml

ENV PHENIX_CONFIG=/etc/phenix-db/config.toml
ENV PHENIX_STORAGE_HOT_TIER_PATH=/data/hot
ENV PHENIX_STORAGE_WARM_TIER_PATH=/data/warm
ENV PHENIX_STORAGE_COLD_TIER_PATH=/data/cold

EXPOSE 50051 8080 9090

CMD ["phenix-server"]
```

```yaml
# docker-compose.yml
version: '3.8'

services:
  phenix-db:
    build: .
    ports:
      - "50051:50051"  # gRPC
      - "8080:8080"    # REST
      - "9090:9090"    # Metrics
    environment:
      - PHENIX_OBSERVABILITY_LOG_LEVEL=info
      - PHENIX_STORAGE_COLD_TIER_PATH=s3://bucket/cold
      - AWS_ACCESS_KEY_ID=${AWS_ACCESS_KEY_ID}
      - AWS_SECRET_ACCESS_KEY=${AWS_SECRET_ACCESS_KEY}
    volumes:
      - hot-tier:/data/hot
      - warm-tier:/data/warm
      - ./phenix-db.toml:/etc/phenix-db/config.toml:ro
    deploy:
      resources:
        limits:
          memory: 128G
          cpus: '32'

volumes:
  hot-tier:
  warm-tier:
```

## Configuration Validation

Phenix-DB validates configuration on startup:

```rust
// Validation errors will be logged
[ERROR] Invalid configuration: polynomial_index.degree must be between 3 and 20
[ERROR] Invalid configuration: tiering.hot_promotion_threshold must be positive
```

## Runtime Configuration Updates

Some configuration can be updated at runtime via API:

```bash
# Update learning rate
curl -X POST http://localhost:8080/api/v1/config/learning/rate \
  -d '{"learning_rate": 0.15}'

# Update tier thresholds
curl -X POST http://localhost:8080/api/v1/config/tiering/thresholds \
  -d '{"hot_promotion": 150.0, "warm_promotion": 15.0}'
```

## Performance Tuning

### For High Throughput

```toml
[polynomial_index]
degree = 3  # Lower degree = faster
branching_factor = 32  # Wider tree = fewer levels

[compression]
target_ratio = 0.7  # Less aggressive compression

[learning]
max_cpu_overhead = 0.02  # Minimal learning overhead
```

### For High Accuracy

```toml
[polynomial_index]
degree = 10  # Higher degree = more accurate
precision_tolerance = 0.000001  # Tighter tolerance

[compression]
target_ratio = 0.95  # Aggressive compression
ramanujan_series_degree = 20

[learning]
convergence_threshold = 0.00001  # More precise
sample_window = 10000  # Larger sample
```

### For Low Latency

```toml
[tiering]
hot_promotion_threshold = 10.0  # Aggressive hot tier promotion
hot_demotion_period_hours = 72  # Keep in hot longer

[bellman_optimizer]
update_interval_sec = 5  # More frequent optimization

[compression]
max_decompression_ms = 1  # Strict latency requirement
```

## Monitoring Configuration

```toml
[observability]
# Detailed metrics for monitoring
enable_mathematical_metrics = true

# Metrics to expose
[observability.metrics]
polynomial_evaluation_time = true
compression_ratio = true
tier_access_latency = true
learning_accuracy = true
cache_hit_rate = true
```

## Security Best Practices

1. **Never commit secrets** to version control
2. **Use environment variables** for sensitive data
3. **Enable TLS** in production
4. **Rotate keys** regularly
5. **Enable audit logging** for compliance
6. **Use KMS** for key management in production

## Troubleshooting

### Configuration Not Loading

```bash
# Check configuration file syntax
phenix-server --validate-config

# Show effective configuration
phenix-server --show-config

# Enable debug logging
PHENIX_OBSERVABILITY_LOG_LEVEL=debug phenix-server
```

### Performance Issues

```bash
# Check current configuration
curl http://localhost:8080/api/v1/config

# View metrics
curl http://localhost:9090/metrics
```

## Next Steps

- Review [Getting Started](./getting-started.md)
- Check [Performance Optimization](./performance.md)
- See [Deployment Guide](../deployment/kubernetes.md)
