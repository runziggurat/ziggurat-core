use crate::geoip::Location;

/// Calculate the distance between two points using Vincenty's inverse formula.
/// The distance is returned in meters.
pub fn geo_get_distance_between(location1: Location, location2: Location) -> f64 {
    let loc1 = geoutils::Location::new(location1.latitude, location1.longitude);
    let loc2 = geoutils::Location::new(location2.latitude, location2.longitude);

    // Calculate the distance between the two points using Vincenty's inverse formula.
    loc1.distance_to(&loc2).map(|d| d.meters()).unwrap_or(0.0)
}

/// Check if the location is in the circle of radius meters.
pub fn geo_is_in_circle(location1: Location, location2: Location, radius: f64) -> bool {
    let distance = geo_get_distance_between(location1, location2);
    distance <= radius
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_distance() {
        let zagreb = Location::new(45.815399, 15.966568);
        let kyiv = Location::new(50.450100, 30.523400);

        let distance = geo_get_distance_between(zagreb, kyiv);
        assert_eq!(distance, 1197101.314);
    }

    #[tokio::test]
    async fn test_being_in_circle() {
        let zagreb = Location::new(45.815399, 15.966568);
        let kyiv = Location::new(50.450100, 30.523400);

        assert!(!geo_is_in_circle(zagreb, kyiv, 1000000.0));
        assert!(geo_is_in_circle(zagreb, kyiv, 2000000.0));
    }

    #[tokio::test]
    async fn test_distance_same_point() {
        let loc1 = Location::new(10.0, 10.0);
        let loc2 = Location::new(10.0, 10.0);

        let distance = geo_get_distance_between(loc1, loc2);
        assert_eq!(distance, 0.0);
    }
}
