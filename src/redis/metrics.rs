use log::{debug, info};
use prometheus::{
    opts, register_counter, register_counter_vec, register_gauge_vec, CounterVec, GaugeVec,
};
use redis::aio::MultiplexedConnection;
use redis::RedisResult;
use std::collections::HashMap;

pub struct Collector {
    conn: MultiplexedConnection,
    target: String,
    target_name: String,

    up: GaugeVec,
    uptime: GaugeVec,
    process_id: GaugeVec,
    io_threads_active: GaugeVec,
    connected_clients: GaugeVec,
    blocked_clients: GaugeVec,
    tracking_clients: GaugeVec,
    clients_in_timeout_table: GaugeVec,
    pubsub_clients: GaugeVec,
    watching_clients: GaugeVec,
    total_watched_keys: GaugeVec,
    total_blocking_keys: GaugeVec,
    total_blocking_keys_on_nokey: GaugeVec,
    client_longest_output_list: GaugeVec,
    client_biggest_input_buf: GaugeVec,
    client_recent_max_output_buffer: GaugeVec,
    client_recent_max_input_buffer: GaugeVec,
    allocator_active: GaugeVec,
    allocator_allocated: GaugeVec,
    allocator_resident: GaugeVec,
    allocator_frag_ratio: GaugeVec,
    allocator_frag_bytes: GaugeVec,
    allocator_rss_ratio: GaugeVec,
    allocator_rss_bytes: GaugeVec,
    used_memory: GaugeVec,
    used_memory_rss: GaugeVec,
    used_memory_peak: GaugeVec,
    used_memory_lua: GaugeVec,
    used_memory_vm_eval: GaugeVec,
    used_memory_scripts_eval: GaugeVec,
    used_memory_overhead: GaugeVec,
    used_memory_startup: GaugeVec,
    used_memory_dataset: GaugeVec,
    number_of_cached_scripts: GaugeVec,
    number_of_functions: GaugeVec,
    number_of_libraries: GaugeVec,
    used_memory_vm_functions: GaugeVec,
    used_memory_scripts: GaugeVec,
    used_memory_functions: GaugeVec,
    used_memory_vm_total: GaugeVec,
    maxmemory: GaugeVec,
    memory_fragmentation_ratio: GaugeVec,
    connected_slaves: GaugeVec,
    keyspace_hits: GaugeVec,
    keyspace_misses: GaugeVec,
    commands_total: GaugeVec,
    commands_rejected_total: GaugeVec,
    commands_duration_total: GaugeVec,
    db_keys: GaugeVec,
    db_expiring_keys: GaugeVec,
    master_last_io_seconds_ago: GaugeVec,

    evicted_keys: CounterVec,
}

impl Collector {
    pub fn new(
        conn: MultiplexedConnection,
        target: &str,
        target_name: &str,
    ) -> Result<Self, prometheus::Error> {
        Ok(Self {
            conn,
            target: target.to_string(),
            target_name: target_name.to_string(),
            up: register_gauge_vec!(
                opts!("up", "Target is online").namespace("redis"),
                &["target", "target_name"]
            )?,
            uptime: register_gauge_vec!(
                opts!("uptime_in_seconds", "Target uptime in seconds").namespace("redis"),
                &["target", "target_name"]
            )?,
            process_id: register_gauge_vec!(
                opts!("process_id", "Redis process ID").namespace("redis"),
                &["target", "target_name"]
            )?,
            io_threads_active: register_gauge_vec!(
                opts!("io_threads_active", "Number of IO threads active").namespace("redis"),
                &["target", "target_name"]
            )?,
            connected_clients: register_gauge_vec!(
                opts!("connected_clients", "Number of connected clients").namespace("redis"),
                &["target", "target_name"]
            )?,
            blocked_clients: register_gauge_vec!(
                opts!("blocked_clients", "Number of blocked clients").namespace("redis"),
                &["target", "target_name"]
            )?,
            tracking_clients: register_gauge_vec!(
                opts!("tracking_clients", "Number of tracking clients").namespace("redis"),
                &["target", "target_name"]
            )?,
            clients_in_timeout_table: register_gauge_vec!(
                opts!(
                    "clients_in_timeout_table",
                    "Number of client in-memory time out"
                )
                .namespace("redis"),
                &["target", "target_name"]
            )?,
            pubsub_clients: register_gauge_vec!(
                opts!("pubsub_clients", "Number of pubsub clients").namespace("redis"),
                &["target", "target_name"]
            )?,
            watching_clients: register_gauge_vec!(
                opts!("watching_clients", "Number of watching clients").namespace("redis"),
                &["target", "target_name"]
            )?,
            total_watched_keys: register_gauge_vec!(
                opts!("total_watched_keys", "Number of total watching keys").namespace("redis"),
                &["target", "target_name"]
            )?,
            total_blocking_keys: register_gauge_vec!(
                opts!("total_blocking_keys", "Number of blocking keys").namespace("redis"),
                &["target", "target_name"]
            )?,
            total_blocking_keys_on_nokey: register_gauge_vec!(
                opts!(
                    "total_blocking_keys_on_nokey",
                    "Number of blocking keys on nokey"
                )
                .namespace("redis"),
                &["target", "target_name"]
            )?,
            client_longest_output_list: register_gauge_vec!(
                opts!(
                    "client_longest_output_list",
                    "Longest output list among clients"
                )
                .namespace("redis"),
                &["target", "target_name"]
            )?,
            client_biggest_input_buf: register_gauge_vec!(
                opts!(
                    "client_biggest_input_buf",
                    "Biggest input buffer among clients"
                )
                .namespace("redis"),
                &["target", "target_name"]
            )?,
            client_recent_max_output_buffer: register_gauge_vec!(
                opts!(
                    "client_recent_max_output_buffer",
                    "Recent maximum output buffer among clients"
                )
                .namespace("redis"),
                &["target", "target_name"]
            )?,
            client_recent_max_input_buffer: register_gauge_vec!(
                opts!(
                    "client_recent_max_input_buffer",
                    "Recent maximum input buffer among clients"
                )
                .namespace("redis"),
                &["target", "target_name"]
            )?,
            allocator_active: register_gauge_vec!(
                opts!("allocator_active", "Total size of allocated memory").namespace("redis"),
                &["target", "target_name"]
            )?,
            allocator_allocated: register_gauge_vec!(
                opts!(
                    "allocator_allocated",
                    "Total size of allocated memory including internal fragmentation"
                )
                .namespace("redis"),
                &["target", "target_name"]
            )?,
            allocator_resident: register_gauge_vec!(
                opts!("allocator_resident", "Total size of resident memory").namespace("redis"),
                &["target", "target_name"]
            )?,
            allocator_frag_ratio: register_gauge_vec!(
                opts!("allocator_frag_ratio", "Ratio of memory fragmentation").namespace("redis"),
                &["target", "target_name"]
            )?,
            allocator_frag_bytes: register_gauge_vec!(
                opts!(
                    "allocator_frag_bytes",
                    "Amount of memory fragmentation in bytes"
                )
                .namespace("redis"),
                &["target", "target_name"]
            )?,
            allocator_rss_ratio: register_gauge_vec!(
                opts!(
                    "allocator_rss_ratio",
                    "Ratio of resident set size to allocated memory"
                )
                .namespace("redis"),
                &["target", "target_name"]
            )?,
            allocator_rss_bytes: register_gauge_vec!(
                opts!("allocator_rss_bytes", "Number of resident set size bytes")
                    .namespace("redis"),
                &["target", "target_name"]
            )?,
            used_memory: register_gauge_vec!(
                opts!(
                    "memory_used_bytes",
                    "Total number of bytes allocated by Redis"
                )
                .namespace("redis"),
                &["target", "target_name"]
            )?,
            used_memory_rss: register_gauge_vec!(
                opts!(
                    "memory_used_rss_bytes",
                    "Number of bytes that Redis allocated as seen by the operating system"
                )
                .namespace("redis"),
                &["target", "target_name"]
            )?,
            used_memory_peak: register_gauge_vec!(
                opts!(
                    "memory_used_peak_bytes",
                    "Peak memory consumed by Redis (in bytes)"
                )
                .namespace("redis"),
                &["target", "target_name"]
            )?,
            used_memory_lua: register_gauge_vec!(
                opts!(
                    "memory_used_lua_bytes",
                    "Number of bytes used by the Lua engine"
                )
                .namespace("redis"),
                &["target", "target_name"]
            )?,
            used_memory_vm_eval: register_gauge_vec!(
                opts!(
                    "memory_used_vm_eval_bytes",
                    "Memory used by VM for evaluation"
                )
                .namespace("redis"),
                &["target", "target_name"]
            )?,
            used_memory_scripts_eval: register_gauge_vec!(
                opts!(
                    "memory_used_scripts_eval_bytes",
                    "Memory used for script evaluation"
                )
                .namespace("redis"),
                &["target", "target_name"]
            )?,
            used_memory_overhead: register_gauge_vec!(
                opts!(
                    "memory_used_overhead_bytes",
                    "The sum in bytes of all overheads allocated by Redis"
                )
                .namespace("redis"),
                &["target", "target_name"]
            )?,
            used_memory_startup: register_gauge_vec!(
                opts!(
                    "memory_used_startup_bytes",
                    "Initial amount of memory consumed by Redis at startup"
                )
                .namespace("redis"),
                &["target", "target_name"]
            )?,
            used_memory_dataset: register_gauge_vec!(
                opts!(
                    "memory_used_dataset_bytes",
                    "The size in bytes of the dataset"
                )
                .namespace("redis"),
                &["target", "target_name"]
            )?,
            number_of_cached_scripts: register_gauge_vec!(
                opts!("number_of_cached_scripts", "Number of cached scripts").namespace("redis"),
                &["target", "target_name"]
            )?,
            number_of_functions: register_gauge_vec!(
                opts!("number_of_functions", "Number of functions").namespace("redis"),
                &["target", "target_name"]
            )?,
            number_of_libraries: register_gauge_vec!(
                opts!("number_of_libraries", "Number of libraries").namespace("redis"),
                &["target", "target_name"]
            )?,
            used_memory_vm_functions: register_gauge_vec!(
                opts!(
                    "memory_used_vm_functions_bytes",
                    "Memory used by VM functions"
                )
                .namespace("redis"),
                &["target", "target_name"]
            )?,
            used_memory_scripts: register_gauge_vec!(
                opts!("memory_used_scripts_bytes", "Memory used by scripts").namespace("redis"),
                &["target", "target_name"]
            )?,
            used_memory_functions: register_gauge_vec!(
                opts!("memory_used_functions_bytes", "Memory used by functions").namespace("redis"),
                &["target", "target_name"]
            )?,
            used_memory_vm_total: register_gauge_vec!(
                opts!("used_memory_vm_total", "Total memory used by VM").namespace("redis"),
                &["target", "target_name"]
            )?,
            maxmemory: register_gauge_vec!(
                opts!("memory_max_bytes", "Maximum amount of memory Redis can use")
                    .namespace("redis"),
                &["target", "target_name"]
            )?,
            memory_fragmentation_ratio: register_gauge_vec!(
                opts!("memory_fragmentation_ratio", "Memory fragmentation ratio")
                    .namespace("redis"),
                &["target", "target_name"]
            )?,

            keyspace_hits: register_gauge_vec!(
                opts!("keyspace_hits_total", "Total number of keyspace hits").namespace("redis"),
                &["target", "target_name"]
            )?,
            keyspace_misses: register_gauge_vec!(
                opts!("keyspace_misses_total", "Total number of keyspace misses")
                    .namespace("redis"),
                &["target", "target_name"]
            )?,

            commands_total: register_gauge_vec!(
                opts!("commands_total", "Total number of calls per command").namespace("redis"),
                &["cmd", "target", "target_name"]
            )?,
            commands_rejected_total: register_gauge_vec!(
                opts!(
                    "commands_rejected_calls_total",
                    "Total number of errors within command execution per command"
                )
                .namespace("redis"),
                &["cmd", "target", "target_name"]
            )?,
            commands_duration_total: register_gauge_vec!(
                opts!(
                    "commands_duration_seconds_total",
                    "Total amount of time in seconds spent per command"
                )
                .namespace("redis"),
                &["cmd", "target", "target_name"]
            )?,
            db_keys: register_gauge_vec!(
                opts!("db_keys", "Total number of keys per DB").namespace("redis"),
                &["db", "target", "target_name"]
            )?,
            db_expiring_keys: register_gauge_vec!(
                opts!("db_keys_expiring", "Total number of expiring keys by DB").namespace("redis"),
                &["db", "target", "target_name"]
            )?,
            master_last_io_seconds_ago: register_gauge_vec!(
                opts!("master_last_io_seconds_ago", "Master last io seconds ago")
                    .namespace("redis"),
                &["target", "target_name"]
            )?,
            connected_slaves: register_gauge_vec!(
                opts!("connected_slaves", "Number of connected slaves").namespace("redis"),
                &["target", "target_name"]
            )?,

            evicted_keys: register_counter_vec!(
                opts!("evicted_keys_total", "Total number of evicted keys").namespace("redis"),
                &["target", "target_name"]
            )?,
        })
    }

    pub async fn collect(&self) -> RedisResult<()> {
        let info: String = redis::cmd("INFO")
            .arg("ALL")
            .query_async(&mut self.conn.clone())
            .await?;

        debug!("Receive data from INFO command");
        self.up
            .with_label_values(&[&self.target, &self.target_name])
            .set(1f64);
        for line in info.lines() {
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((metric, value)) = line.split_once(':') {
                if metric.starts_with("cmdstat_") {
                    self.add_cmdstat(metric, value);
                    continue;
                }

                if let Some(metric) = self.parse_gauge_metric_name(metric) {
                    let value = value.parse::<f64>().unwrap_or(0f64);
                    metric
                        .with_label_values(&[&self.target, &self.target_name])
                        .set(value);
                    continue;
                }

                if let Some(metric) = self.parse_counter_metric_name(metric) {
                    let value = value.parse::<f64>().unwrap_or(0f64);
                    metric
                        .with_label_values(&[&self.target, &self.target_name])
                        .inc_by(value);
                    continue;
                }

                if line.starts_with("db") {
                    let (db, stats) = line.split_once(':').unwrap();
                    let stats_map: HashMap<&str, &str> =
                        stats.split(',').filter_map(|s| s.split_once('=')).collect();

                    if let Some(keys) = stats_map.get("keys") {
                        if let Ok(value) = keys.parse::<f64>() {
                            self.db_keys
                                .with_label_values(&[db, &self.target, &self.target_name])
                                .set(value);
                        }
                    }

                    if let Some(keys) = stats_map.get("expires") {
                        if let Ok(value) = keys.parse::<f64>() {
                            self.db_expiring_keys
                                .with_label_values(&[db, &self.target, &self.target_name])
                                .set(value);
                        }
                    }
                    continue;
                }

                debug!("Metric {} not found", metric);
            }
        }

        debug!("Metrics collected");
        Ok(())
    }

    fn add_cmdstat(&self, metric: &str, value: &str) {
        let cmd = metric.strip_prefix("cmdstat_").unwrap();

        let stats_map: HashMap<&str, &str> =
            value.split(',').filter_map(|s| s.split_once('=')).collect();

        if let Some(calls) = stats_map.get("calls") {
            if let Ok(value) = calls.parse::<f64>() {
                self.commands_total
                    .with_label_values(&[&cmd, &self.target, &self.target_name])
                    .set(value);
            }
        }

        if let Some(rejected) = stats_map.get("rejected_calls") {
            if let Ok(value) = rejected.parse::<f64>() {
                self.commands_rejected_total
                    .with_label_values(&[&cmd, &self.target, &self.target_name])
                    .set(value);
            }
        }

        if let Some(usec) = stats_map.get("usec") {
            if let Ok(microseconds) = usec.parse::<f64>() {
                let seconds = microseconds / 1_000_000.0;
                self.commands_duration_total
                    .with_label_values(&[&cmd, &self.target, &self.target_name])
                    .set(seconds);
            }
        }
    }

    fn parse_gauge_metric_name(&self, name: &str) -> Option<GaugeVec> {
        match name {
            "up" => Some(self.up.clone()),
            "uptime_in_seconds" => Some(self.uptime.clone()),
            "process_id" => Some(self.process_id.clone()),
            "io_threads_active" => Some(self.io_threads_active.clone()),
            "connected_clients" => Some(self.connected_clients.clone()),
            "blocked_clients" => Some(self.blocked_clients.clone()),
            "tracking_clients" => Some(self.tracking_clients.clone()),
            "clients_in_timeout_table" => Some(self.clients_in_timeout_table.clone()),
            "pubsub_clients" => Some(self.pubsub_clients.clone()),
            "watching_clients" => Some(self.watching_clients.clone()),
            "total_watched_keys" => Some(self.total_watched_keys.clone()),
            "total_blocking_keys" => Some(self.total_blocking_keys.clone()),
            "total_blocking_keys_on_nokey" => Some(self.total_blocking_keys_on_nokey.clone()),
            "client_longest_output_list" => Some(self.client_longest_output_list.clone()),
            "client_biggest_input_buf" => Some(self.client_biggest_input_buf.clone()),
            "client_recent_max_output_buffer" => Some(self.client_recent_max_output_buffer.clone()),
            "client_recent_max_input_buffer" => Some(self.client_recent_max_input_buffer.clone()),
            "allocator_active" => Some(self.allocator_active.clone()),
            "allocator_allocated" => Some(self.allocator_allocated.clone()),
            "allocator_resident" => Some(self.allocator_resident.clone()),
            "allocator_frag_ratio" => Some(self.allocator_frag_ratio.clone()),
            "allocator_frag_bytes" => Some(self.allocator_frag_bytes.clone()),
            "allocator_rss_ratio" => Some(self.allocator_rss_ratio.clone()),
            "allocator_rss_bytes" => Some(self.allocator_rss_bytes.clone()),
            "used_memory" => Some(self.used_memory.clone()),
            "used_memory_rss" => Some(self.used_memory_rss.clone()),
            "used_memory_peak" => Some(self.used_memory_peak.clone()),
            "used_memory_lua" => Some(self.used_memory_lua.clone()),
            "used_memory_vm_eval" => Some(self.used_memory_vm_eval.clone()),
            "used_memory_scripts_eval" => Some(self.used_memory_scripts_eval.clone()),
            "used_memory_overhead" => Some(self.used_memory_overhead.clone()),
            "used_memory_startup" => Some(self.used_memory_startup.clone()),
            "used_memory_dataset" => Some(self.used_memory_dataset.clone()),
            "number_of_cached_scripts" => Some(self.number_of_cached_scripts.clone()),
            "number_of_functions" => Some(self.number_of_functions.clone()),
            "number_of_libraries" => Some(self.number_of_libraries.clone()),
            "used_memory_vm_functions" => Some(self.used_memory_vm_functions.clone()),
            "used_memory_scripts" => Some(self.used_memory_scripts.clone()),
            "used_memory_functions" => Some(self.used_memory_functions.clone()),
            "used_memory_vm_total" => Some(self.used_memory_vm_total.clone()),
            "maxmemory" => Some(self.maxmemory.clone()),
            "mem_fragmentation_ratio" => Some(self.memory_fragmentation_ratio.clone()),
            "keyspace_hits" => Some(self.keyspace_hits.clone()),
            "keyspace_misses" => Some(self.keyspace_misses.clone()),
            "master_last_io_seconds_ago" => Some(self.master_last_io_seconds_ago.clone()),
            "connected_slaves" => Some(self.connected_slaves.clone()),
            _ => None,
        }
    }

    fn parse_counter_metric_name(&self, name: &str) -> Option<CounterVec> {
        match name {
            "evicted_keys" => Some(self.evicted_keys.clone()),
            _ => None,
        }
    }
}
