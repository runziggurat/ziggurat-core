use std::net::IpAddr;

use async_trait::async_trait;

///
#[async_trait]
pub trait GeoIPService {
    async fn lookup(&self, ip: IpAddr) -> Result<GeoIPInfo, String>;
}

/// Geographical location of an IP address.
#[derive(Copy, Clone, PartialEq)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

/// IP information
#[derive(Clone)]
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
