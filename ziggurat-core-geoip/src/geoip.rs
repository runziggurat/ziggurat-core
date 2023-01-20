//use ipinfo::{IpInfo, IpInfoConfig};   // TODO(asmie): implement IpInfo provider
use ip2location::{Record, DB};
use ipgeolocate::{Locator, Service};

/// List of supported geoip providers.
pub enum Provider {
    IpApiCom,
    IpApiCo,
    IpInfo,
    Ip2Location,
}

/// Geoip provider service configuration.
pub struct GeoIPService {
    /// Geoip provider. Look at `Provider` enum for supported providers.
    pub provider: Provider,
    /// API key (paid or free) for online providers. For offline providers put here path to database file.
    pub api_key: String,
}

/// IP information
pub struct IPGeoInfo {
    /// IP address
    pub ip: String,
    /// Country name (long name)
    pub country: Option<String>,
    /// City name
    pub city: Option<String>,
    /// IP address latitude
    pub latitude: Option<f64>,
    /// IP address longitude
    pub longitude: Option<f64>,
    /// Timezone of the IP
    pub timezone: Option<String>,
    /// ISP name (unavailable for some providers)
    pub isp: Option<String>,
}

impl GeoIPService {
    pub fn new(provider: Provider, api_key: String) -> Self {
        Self { provider, api_key }
    }

    pub async fn lookup_ip(&self, ip: String) -> Result<IPGeoInfo, String> {
        match self.provider {
            Provider::IpInfo => {
                unimplemented!("IpInfo not implemented yet");
            }
            Provider::IpApiCom | Provider::IpApiCo => {
                let service = match self.provider {
                    Provider::IpApiCom => Service::IpApi,
                    Provider::IpApiCo => Service::IpApiCo,
                    _ => unreachable!(),
                };

                match Locator::get(ip.as_str(), service).await {
                    Ok(loc_ip) => Ok(IPGeoInfo {
                        ip,
                        country: Some(loc_ip.country),
                        city: Some(loc_ip.city),
                        latitude: match loc_ip.latitude.parse::<f64>() {
                            Ok(lat) => Some(lat),
                            Err(_) => None,
                        },
                        longitude: match loc_ip.longitude.parse::<f64>() {
                            Ok(long) => Some(long),
                            Err(_) => None,
                        },
                        timezone: Some(loc_ip.timezone),
                        isp: Some("".to_string()),
                    }),
                    Err(error) => Err(error.to_string()),
                }
            }
            Provider::Ip2Location => {
                let mut db = DB::from_file(self.api_key.clone()).expect("Failed to open database");
                let record = db.ip_lookup(ip.parse().expect("Failed to parse IP address"));
                let record = if let Ok(Record::LocationDb(rec)) = record {
                    Some(rec)
                } else {
                    None
                };
                if record.is_some() {
                    let record = record.unwrap();
                    Ok(IPGeoInfo {
                        ip,
                        country: match record.country {
                            Some(country) => Some(country.long_name),
                            None => None,
                        },
                        city: record.city,
                        latitude: record.latitude.map(|lat| lat as f64),
                        longitude: record.longitude.map(|long| long as f64),
                        timezone: record.time_zone,
                        isp: record.isp,
                    })
                } else {
                    Err("Failed to get record".to_string())
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ip_api_com() {
        let geoip = GeoIPService::new(Provider::IpApiCom, "".to_string());
        let ipgeo = geoip.lookup_ip("8.8.8.8".to_string()).await.unwrap();
        assert_eq!(ipgeo.country.unwrap(), "United States");
        assert_eq!(ipgeo.city.unwrap(), "Ashburn");
        assert_eq!(ipgeo.latitude.unwrap(), 39.03);
        assert_eq!(ipgeo.longitude.unwrap(), -77.5);
        assert_eq!(ipgeo.timezone.unwrap(), "America/New_York");
    }

    #[tokio::test]
    async fn test_ip_api_co() {
        let geoip = GeoIPService::new(Provider::IpApiCo, "".to_string());
        let ipgeo = geoip.lookup_ip("8.8.8.8".to_string()).await.unwrap();
        assert_eq!(ipgeo.country.unwrap(), "United States");
        assert_eq!(ipgeo.city.unwrap(), "Mountain View");
        assert_eq!(ipgeo.latitude.unwrap(), 37.42301);
        assert_eq!(ipgeo.longitude.unwrap(), -122.083352);
        assert_eq!(ipgeo.timezone.unwrap(), "America/Los_Angeles");
    }
}
