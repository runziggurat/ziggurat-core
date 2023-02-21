use std::net::IpAddr;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::coordinates::Coordinates;

/// Every provider need to implement this trait.
#[async_trait]
pub trait GeoIPService {
    /// Lookup the IP address and return the GeoIPInfo.
    async fn lookup(&self, ip: IpAddr) -> Result<GeoIPInfo, String>;
}

/// GeoIP information.
#[derive(Clone, Deserialize, Serialize)]
pub struct GeoIPInfo {
    /// IP address
    pub ip: IpAddr,
    /// GeoInfo struct
    pub geo_info: GeoInfo,
}

/// Geo information
#[derive(Clone, Deserialize, Serialize)]
pub struct GeoInfo {
    /// Country name (long name)
    pub country: Option<String>,
    /// City name
    pub city: Option<String>,
    /// Location of the IP address
    pub coordinates: Option<Coordinates>,
    /// Timezone of the IP
    pub timezone: Option<String>,
    /// ISP name (unavailable for some providers)
    pub isp: Option<String>,
}
