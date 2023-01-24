use std::net::IpAddr;

use async_trait::async_trait;
use ip2location::{Record, DB};

use crate::geoip::{GeoIPInfo, GeoIPService};

/// Ip2Location provider service configuration.
#[derive(Copy, Clone)]
pub struct Ip2LocationService {
    /// Database file path.
    pub database_file: &'static str,
}

impl Ip2LocationService {
    pub fn new(database_file: &'static str) -> Self {
        Self { database_file }
    }
}

#[async_trait]
impl GeoIPService for Ip2LocationService {
    async fn lookup(&self, ip: IpAddr) -> Result<GeoIPInfo, String> {
        let db = DB::from_file(self.database_file);
        if db.is_err() {
            return Err("failed to open the database".to_string());
        }

        let record = db.unwrap().ip_lookup(ip);
        let record = if let Ok(Record::LocationDb(rec)) = record {
            rec
        } else {
            return Err("failed to get the record".to_string());
        };

        Ok(GeoIPInfo {
            ip,
            country: record.country.map(|c| c.long_name),
            city: record.city,
            latitude: record.latitude.map(|lat| lat as f64),
            longitude: record.longitude.map(|long| long as f64),
            timezone: record.time_zone,
            isp: record.isp,
        })
    }
}
