use postgres_types::{ToSql, FromSql};
use systemstat::{System, Platform, saturating_sub_bytes};
use std::time::Duration;

use crate::config::global::Config;
use crate::database::query_db::{QueryFunction, query_db};


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct SaiStatsServer {
    pub uploaded: i64,
    pub downloaded: i64,
    pub disk_used: i64,
    pub disk_free: i64,
    pub cpu: f64,
    pub memory: f64,
}

impl QueryFunction for SaiStatsServer {
    fn query(&self) -> &str {
        "SELECT as_stats_server($1)"
    }
}


pub async fn measure_stats_server(
    config: &Config,
    netif_name: &str,
    last_bytes_received: &i64,
    last_bytes_sent: &i64,
) -> (i64, i64) {

    let sys = System::new();

    // Network
    let netstats = sys.network_stats(netif_name).unwrap();
    let bytes_received = netstats.rx_bytes.as_u64() as i64;
    let bytes_sent = netstats.tx_bytes.as_u64() as i64;
    let uploaded_value = bytes_sent - last_bytes_sent;
    let downloaded_value = bytes_received - last_bytes_received;

    // Disk used and free
    let mount = sys.mount_at("/").unwrap();
    let disk_free: i64 = mount.avail.as_u64() as i64;
    let disk_total: i64  = mount.total.as_u64() as i64;
    let disk_used = disk_total - disk_free;

    // CPU
    let cpu_agg = sys.cpu_load_aggregate().unwrap();
    tokio::time::sleep(Duration::from_secs(1)).await;
    let cpu = cpu_agg.done().unwrap().user;
    let cpu_value = (cpu as f64) * 100.0;

    // Memory
    let mem = sys.memory().unwrap();
    let mem_total = mem.total.as_u64() as f64;
    let mem_used = saturating_sub_bytes(mem.total, mem.free).as_u64() as f64;
    let memory_value = (mem_used / mem_total) * 100.0;

    match query_db(
        config,
        SaiStatsServer {
            uploaded: uploaded_value,
            downloaded: downloaded_value,
            disk_used: disk_used,
            disk_free: disk_free,
            cpu: cpu_value,
            memory: memory_value,
        },
    ).await {

        Ok(_row) => {},

        Err(e) => {
            panic!("SSTATS ERROR: {}", e);
        },

    };

    (bytes_received, bytes_sent)

}

