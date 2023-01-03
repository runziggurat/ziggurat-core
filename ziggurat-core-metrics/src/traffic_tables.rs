use histogram::Histogram;
use tabled::{Table, Tabled};

use crate::tables::{fmt_table, table_float_display};

/// Provides a simplified interface to produce a well-formatted table for traffic statistics.
///
/// Table can be displayed by `println!("{}", table)`
#[derive(Default)]
pub struct TrafficRequestsTable {
    rows: Vec<TrafficRequestStats>,
}

#[derive(Tabled, Default, Debug, Clone)]
pub struct TrafficRequestStats {
    #[tabled(rename = " normal peers ")]
    normal_peers: u16,
    #[tabled(rename = " high-traffic peers ")]
    high_traffic_peers: u16,
    #[tabled(rename = " requests ")]
    requests: u16,
    #[tabled(rename = " min (ms) ")]
    latency_min: u16,
    #[tabled(rename = " max (ms) ")]
    latency_max: u16,
    #[tabled(rename = " std dev (ms) ")]
    latency_std_dev: u16,
    #[tabled(rename = " completion % ")]
    #[tabled(display_with = "table_float_display")]
    completion: f64,
    #[tabled(rename = " time (s) ")]
    #[tabled(display_with = "table_float_display")]
    time: f64,
}

impl TrafficRequestStats {
    pub fn new(
        normal_peers: u16,
        high_traffic_peers: u16,
        requests: u16,
        latency: Histogram,
        time: f64,
    ) -> Self {
        Self {
            normal_peers,
            high_traffic_peers,
            requests,
            completion: (latency.entries() as f64) / (normal_peers as f64 * requests as f64)
                * 100.00,
            latency_min: latency.minimum().unwrap() as u16,
            latency_max: latency.maximum().unwrap() as u16,
            latency_std_dev: latency.stddev().unwrap() as u16,
            time,
        }
    }
}

impl TrafficRequestsTable {
    pub fn add_row(&mut self, row: TrafficRequestStats) {
        self.rows.push(row);
    }
}

impl std::fmt::Display for TrafficRequestsTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&fmt_table(Table::new(&self.rows)))
    }
}
