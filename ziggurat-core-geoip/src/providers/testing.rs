use std::net::IpAddr;

use async_trait::async_trait;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::geoip::{GeoIPInfo, GeoIPService};

/// List of supported testing providers.
#[derive(Copy, Clone, PartialEq)]
pub enum TestingProvider {
    /// Return empty data.
    Zeroed,
    /// Return random data.
    Random,
}

/// Testing provider service configuration.
#[derive(Copy, Clone)]
pub struct TestingService {
    /// Testing provider.
    pub provider: TestingProvider,
}

impl TestingService {
    pub fn new(provider: TestingProvider) -> Self {
        Self { provider }
    }
}

#[async_trait]
impl GeoIPService for TestingService {
    async fn lookup(&self, ip: IpAddr) -> Result<GeoIPInfo, String> {
        if self.provider == TestingProvider::Zeroed {
            Ok(GeoIPInfo {
                ip,
                country: Some("".to_owned()),
                city: Some("".to_owned()),
                latitude: Some(0.0),
                longitude: Some(0.0),
                timezone: Some("".to_owned()),
                isp: Some("".to_owned()),
            })
        } else {
            Ok(GeoIPInfo {
                ip,
                country: Some(
                    thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(8)
                        .map(char::from)
                        .collect(),
                ),
                city: Some(
                    thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(8)
                        .map(char::from)
                        .collect(),
                ),
                latitude: Some(thread_rng().gen_range(-90.0..90.0)),
                longitude: Some(thread_rng().gen_range(-180.0..180.0)),
                timezone: Some(
                    thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(8)
                        .map(char::from)
                        .collect(),
                ),
                isp: Some(
                    thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(8)
                        .map(char::from)
                        .collect(),
                ),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_testing_provider() {
        let geoip = TestingService::new(TestingProvider::Zeroed);
        let ipgeo = geoip.lookup("8.8.8.8".parse().unwrap()).await.unwrap();
        assert_eq!(ipgeo.country.unwrap(), "");
        assert_eq!(ipgeo.city.unwrap(), "");
        assert_eq!(ipgeo.latitude.unwrap(), 0.0);
        assert_eq!(ipgeo.longitude.unwrap(), 0.0);
        assert_eq!(ipgeo.timezone.unwrap(), "");
    }
}
