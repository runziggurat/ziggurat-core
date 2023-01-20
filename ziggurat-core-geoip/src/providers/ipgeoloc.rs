use std::net::IpAddr;

use async_trait::async_trait;
use ipgeolocate::{Locator, Service};

use crate::geoip::{GeoIPInfo, GeoIPService};

/// List of supported ipgeolocate providers.
#[derive(Copy, Clone, PartialEq)]
pub enum BackendProvider {
    IpApiCom,
    IpApiCo,
}

/// ipgeolocate provider service configuration.
#[derive(Copy, Clone)]
pub struct IpGeolocateService {
    /// Geoip provider.
    pub provider: BackendProvider,
    /// API key.
    pub api_key: &'static str,
}

impl IpGeolocateService {
    pub fn new(provider: BackendProvider, api_key: &'static str) -> Self {
        Self { provider, api_key }
    }
}

#[async_trait]
impl GeoIPService for IpGeolocateService {
    async fn lookup(&self, ip: IpAddr) -> Result<GeoIPInfo, String> {
        let service = if self.provider == BackendProvider::IpApiCom {
            Service::IpApi
        } else {
            Service::IpApiCo
        };

        match Locator::get(ip.to_string().as_str(), service).await {
            Ok(loc_ip) => Ok(GeoIPInfo {
                ip,
                country: Some(loc_ip.country),
                city: Some(loc_ip.city),
                latitude: loc_ip.latitude.parse::<f64>().ok(),
                longitude: loc_ip.longitude.parse::<f64>().ok(),
                timezone: Some(loc_ip.timezone),
                isp: Some("".to_owned()),
            }),
            Err(error) => Err(error.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ip_api_com() {
        let geoip = IpGeolocateService::new(BackendProvider::IpApiCom, "");
        let ipgeo = geoip.lookup("8.8.8.8".parse().unwrap()).await.unwrap();
        assert_eq!(ipgeo.country.unwrap(), "United States");
        assert_eq!(ipgeo.city.unwrap(), "Ashburn");
        assert_eq!(ipgeo.latitude.unwrap(), 39.03);
        assert_eq!(ipgeo.longitude.unwrap(), -77.5);
        assert_eq!(ipgeo.timezone.unwrap(), "America/New_York");
    }

    #[tokio::test]
    async fn test_ip_api_co() {
        let geoip = IpGeolocateService::new(BackendProvider::IpApiCo, "");
        let ipgeo = geoip.lookup("8.8.8.8".parse().unwrap()).await.unwrap();
        assert_eq!(ipgeo.country.unwrap(), "United States");
        assert_eq!(ipgeo.city.unwrap(), "Mountain View");
        assert_eq!(ipgeo.latitude.unwrap(), 37.42301);
        assert_eq!(ipgeo.longitude.unwrap(), -122.083352);
        assert_eq!(ipgeo.timezone.unwrap(), "America/Los_Angeles");
    }
}
