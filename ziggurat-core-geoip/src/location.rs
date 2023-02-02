use serde::{Deserialize, Serialize};

/// Geographical location
#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct Location {
    /// Latitude value
    pub latitude: f64,
    /// Longitude value
    pub longitude: f64,
}

impl Location {
    /// Create a new Location struct.
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }

    /// Calculate the distance between two points using Vincenty's inverse formula.
    /// The distance is returned in meters.
    pub fn distance_to(&self, location: Location) -> f64 {
        let loc1 = geoutils::Location::new(self.latitude, self.longitude);
        let loc2 = geoutils::Location::new(location.latitude, location.longitude);

        // Calculate the distance between the two points using Vincenty's inverse formula.
        loc1.distance_to(&loc2).map(|d| d.meters()).unwrap_or(0.0)
    }

    /// Check if the location is in the circle of radius meters.
    pub fn is_in_circle(&self, location: Location, radius: f64) -> bool {
        let distance = self.distance_to(location);
        distance <= radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_distance() {
        let zagreb = Location::new(45.815399, 15.966568);
        let kyiv = Location::new(50.450100, 30.523400);

        let distance = zagreb.distance_to(kyiv);
        assert_eq!(distance, 1197101.314);
    }

    #[tokio::test]
    async fn test_being_in_circle() {
        let zagreb = Location::new(45.815399, 15.966568);
        let kyiv = Location::new(50.450100, 30.523400);

        assert!(!zagreb.is_in_circle(kyiv, 1000000.0));
        assert!(zagreb.is_in_circle(kyiv, 2000000.0));
    }

    #[tokio::test]
    async fn test_distance_same_point() {
        let loc1 = Location::new(10.0, 10.0);
        let loc2 = Location::new(10.0, 10.0);

        let distance = loc1.distance_to(loc2);
        assert_eq!(distance, 0.0);
    }
}
