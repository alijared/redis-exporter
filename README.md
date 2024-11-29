# Redis Exporter

A Prometheus exporter for Redis metrics written in Rust. This exporter collects various Redis metrics and exposes them in Prometheus format, allowing you to monitor your Redis instances using Prometheus and Grafana.

## Features

- Monitors multiple Redis instances simultaneously
- Collects comprehensive Redis metrics including:
  - Memory usage and fragmentation
  - Client connections
  - Command statistics
  - Keyspace statistics
  - Replication status
  - Database keys and expired keys
  - Performance metrics
- Configurable collection intervals
- Customizable target naming
- Connection timeout handling
- Graceful shutdown support

## Installation

### From Source

```bash
git clone https://github.com/yourusername/redis-exporter
cd redis-exporter
cargo build --release
```

The compiled binary will be available at `target/release/redis-exporter`

## Usage

```bash
redis-exporter [OPTIONS]
```

### Command Line Options

- `-c, --config-filepath <PATH>`: Path to the configuration file (default: "config.yaml")
- `--log-level <LEVEL>`: Set the log level (default: info)
- `-h, --help`: Print help
- `-V, --version`: Print version

## Configuration

Create a `config.yaml` file with the following structure:

```yaml
# Port where the Prometheus metrics will be exposed
prometheus_port: 9090

# Interval in seconds between metric collections
collect_interval: 15

# Redis targets to monitor
targets:
  - name: "prod-redis-1"  # Optional friendly name
    url: "redis://redis1.example.com:6379"
    response_timeout_ms: 5000
    connection_timeout_ms: 5000
  
  - name: "prod-redis-2"
    url: "redis://redis2.example.com:6379"
    response_timeout_ms: 5000
    connection_timeout_ms: 5000
```

### Configuration Options

- `prometheus_port`: The port where the exporter will expose metrics (default: 9090)
- `collect_interval`: How often to collect metrics in seconds
- `targets`: List of Redis instances to monitor
  - `name`: Optional friendly name for the instance (defaults to URL if not specified)
  - `url`: Redis connection URL
  - `response_timeout_ms`: Timeout for Redis commands in milliseconds (default: 5000)
  - `connection_timeout_ms`: Timeout for establishing connections in milliseconds (default: 5000)

## Metrics

The exporter exposes various Redis metrics with the prefix `redis_`. Each metric includes labels for:
- `target`: The Redis instance URL
- `target_name`: The friendly name of the instance (if configured)

Some of the key metrics include:

- `redis_up`: Shows if the target is online (1) or offline (0)
- `redis_connected_clients`: Number of connected clients
- `redis_memory_used_bytes`: Total memory used by Redis
- `redis_commands_total`: Total number of commands processed
- `redis_keyspace_hits_total`: Number of successful lookups of keys in the main dictionary
- `redis_keyspace_misses_total`: Number of failed lookups of keys
- `redis_memory_fragmentation_ratio`: Ratio of memory allocation overhead

## Prometheus Configuration

Add the following to your `prometheus.yml`:

```yaml
scrape_configs:
  - job_name: 'redis'
    static_configs:
      - targets: ['localhost:9090']
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT [LICENSE](LICENSE) - see the LICENSE file for details.
