use tabled::Tabled;

use crate::tables::table_float_display;

#[derive(Tabled, Default, Debug, Clone)]
struct ConnectionStats {
    #[tabled(rename = "\n max peers ")]
    pub max_peers: u16,
    #[tabled(rename = "\n peers ")]
    pub peers: u16,
    #[tabled(rename = " connection \n accepted ")]
    pub accepted: u16,
    #[tabled(rename = " connection \n rejected ")]
    pub rejected: u16,
    #[tabled(rename = " connection \n terminated ")]
    pub terminated: u16,
    #[tabled(rename = " connection \n error ")]
    pub conn_error: u16,
    #[tabled(rename = " connection \n timed out ")]
    pub timed_out: u16,
    #[tabled(rename = "\n time (s) ")]
    #[tabled(display_with = "table_float_display")]
    pub time: f64,
}

impl ConnectionStats {
    pub fn new(max_peers: u16, peers: u16) -> Self {
        Self {
            max_peers,
            peers,
            ..Default::default()
        }
    }
}
