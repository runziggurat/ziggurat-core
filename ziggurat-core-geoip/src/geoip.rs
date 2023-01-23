use std::net::IpAddr;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

/// Every provider need to implement this trait.
#[async_trait]
pub trait GeoIPService {
    /// Lookup the IP address and return the GeoIPInfo.
    async fn lookup(&self, ip: IpAddr) -> Result<GeoIPInfo, String>;
}

/// IP information
#[derive(Clone, Deserialize, Serialize)]
pub struct GeoIPInfo {
    /// IP address
    pub ip: IpAddr,
    /// Country name (long name)
    pub country: Option<String>,
    /// City name
    pub city: Option<String>,
    /// Latitude
    pub latitude: Option<f64>,
    /// Longitude
    pub longitude: Option<f64>,
    /// Timezone of the IP
    pub timezone: Option<String>,
    /// ISP name (unavailable for some providers)
    pub isp: Option<String>,
}
