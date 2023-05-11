use std::net::IpAddr;

use async_trait::async_trait;
use ip2location::{Record, DB};

use crate::{
    coordinates::Coordinates,
    geoip::{GeoIPInfo, GeoIPService, GeoInfo},
};

/// Ip2Location provider service configuration.
#[derive(Clone)]
pub struct Ip2LocationService {
    /// Database file path.
    pub database_file: String,

    /// For LITE versions IPv4 and IPv6 databases are separated. If you want to use IPv6 database,
    /// you need to specify the path to the IPv6 database file.
    /// NOTE: commercial version have both IPv4 and IPv6 data in one database file. When using
    /// commercial version, you don't need to specify this field.
    pub database_file_ipv6: Option<String>,
}

impl Ip2LocationService {
    pub fn new(database_file: &str, database_file_ipv6: Option<String>) -> Self {
        Self {
            database_file: database_file.to_owned(),
            database_file_ipv6,
        }
    }
}

#[async_trait]
impl GeoIPService for Ip2LocationService {
    async fn lookup(&self, ip: IpAddr) -> Result<GeoIPInfo, String> {
        let db_file = match ip {
            IpAddr::V4(_) => &self.database_file,
            IpAddr::V6(_) => self
                .database_file_ipv6
                .as_ref()
                .unwrap_or(&self.database_file),
        };

        let mut db = DB::from_file(db_file).map_err(|_| "database file can't be loaded")?;

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
                coordinates: match (record.latitude, record.longitude) {
                    (Some(lat), Some(long)) => Some(Coordinates {
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
