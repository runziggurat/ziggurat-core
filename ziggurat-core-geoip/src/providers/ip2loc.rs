use std::net::IpAddr;

use async_trait::async_trait;
use ip2location::{Record, DB};

use crate::{
    geoip::{GeoIPInfo, GeoIPService, GeoInfo},
    location::Location,
};

/// Ip2Location provider service configuration.
#[derive(Clone)]
pub struct Ip2LocationService {
    /// Database file path.
    pub database_file: String,
}

impl Ip2LocationService {
    pub fn new(database_file: &str) -> Self {
        Self {
            database_file: database_file.to_owned(),
        }
    }
}

#[async_trait]
impl GeoIPService for Ip2LocationService {
    async fn lookup(&self, ip: IpAddr) -> Result<GeoIPInfo, String> {
        let mut db =
            DB::from_file(&self.database_file).map_err(|_| "database file can't be loaded")?;

        let record = db.ip_lookup(ip);
        let record = if let Ok(Record::LocationDb(rec)) = record {
            rec
        } else {
            return Err("failed to get the record".to_string());
        };

        Ok(GeoIPInfo {
            ip,
            geo_info: GeoInfo {
                country: record.country.map(|c| c.long_name),
                city: record.city,
                location: match (record.latitude, record.longitude) {
                    (Some(lat), Some(long)) => Some(Location {
                        latitude: lat as f64,
                        longitude: long as f64,
                    }),
                    _ => None,
                },
                timezone: record.time_zone,
                isp: record.isp,
            },
        })
    }
}
