use std::f64::consts::PI;

/// Calculate the distance between two points using the Haversine formula
pub fn calculate_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6371000.0; // Earth's radius in meters
    
    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let delta_lat = (lat2 - lat1).to_radians();
    let delta_lon = (lon2 - lon1).to_radians();
    
    let a = (delta_lat / 2.0).sin() * (delta_lat / 2.0).sin() +
            lat1_rad.cos() * lat2_rad.cos() *
            (delta_lon / 2.0).sin() * (delta_lon / 2.0).sin();
    
    let c = 2.0 * a.sqrt().asin();
    
    r * c
}

/// Calculate the bearing between two points
pub fn calculate_bearing(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let delta_lon = (lon2 - lon1).to_radians();
    
    let y = delta_lon.sin() * lat2_rad.cos();
    let x = lat1_rad.cos() * lat2_rad.sin() - lat1_rad.sin() * lat2_rad.cos() * delta_lon.cos();
    
    let bearing = y.atan2(x).to_degrees();
    
    // Normalize to 0-360
    (bearing + 360.0) % 360.0
}

/// Calculate a point at a given distance and bearing from a starting point
pub fn calculate_destination(lat: f64, lon: f64, bearing: f64, distance: f64) -> (f64, f64) {
    let r = 6371000.0; // Earth's radius in meters
    
    let lat_rad = lat.to_radians();
    let lon_rad = lon.to_radians();
    let bearing_rad = bearing.to_radians();
    
    let angular_distance = distance / r;
    
    let lat2_rad = (lat_rad.sin() * angular_distance.cos() +
                    lat_rad.cos() * angular_distance.sin() * bearing_rad.cos()).asin();
    
    let lon2_rad = lon_rad + (bearing_rad.sin() * angular_distance.sin() * lat_rad.cos()).atan2(
        angular_distance.cos() - lat_rad.sin() * lat2_rad.sin()
    );
    
    (lat2_rad.to_degrees(), lon2_rad.to_degrees())
}

/// Convert speed from m/s to km/h
pub fn ms_to_kmh(speed_ms: f64) -> f64 {
    speed_ms * 3.6
}

/// Convert speed from km/h to m/s
pub fn kmh_to_ms(speed_kmh: f64) -> f64 {
    speed_kmh / 3.6
}

/// Convert speed from m/s to mph
pub fn ms_to_mph(speed_ms: f64) -> f64 {
    speed_ms * 2.23694
}

/// Convert speed from mph to m/s
pub fn mph_to_ms(speed_mph: f64) -> f64 {
    speed_mph / 2.23694
}

/// Calculate the total g-force magnitude from x, y, z components
pub fn calculate_g_force_magnitude(gx: f64, gy: f64, gz: f64) -> f64 {
    (gx * gx + gy * gy + gz * gz).sqrt()
}

/// Calculate acceleration from speed values over time
pub fn calculate_acceleration(speed1: f64, speed2: f64, time_delta: f64) -> f64 {
    if time_delta == 0.0 {
        return 0.0;
    }
    (speed2 - speed1) / time_delta
}

/// Convert coordinates from WGS84 to a local coordinate system
pub fn wgs84_to_local(lat: f64, lon: f64, ref_lat: f64, ref_lon: f64) -> (f64, f64) {
    let r = 6371000.0; // Earth's radius in meters
    
    let lat_rad = lat.to_radians();
    let lon_rad = lon.to_radians();
    let ref_lat_rad = ref_lat.to_radians();
    let ref_lon_rad = ref_lon.to_radians();
    
    let delta_lat = lat_rad - ref_lat_rad;
    let delta_lon = lon_rad - ref_lon_rad;
    
    let x = delta_lon * r * ref_lat_rad.cos();
    let y = delta_lat * r;
    
    (x, y)
}

/// Convert coordinates from local coordinate system to WGS84
pub fn local_to_wgs84(x: f64, y: f64, ref_lat: f64, ref_lon: f64) -> (f64, f64) {
    let r = 6371000.0; // Earth's radius in meters
    
    let ref_lat_rad = ref_lat.to_radians();
    let ref_lon_rad = ref_lon.to_radians();
    
    let delta_lat = y / r;
    let delta_lon = x / (r * ref_lat_rad.cos());
    
    let lat_rad = ref_lat_rad + delta_lat;
    let lon_rad = ref_lon_rad + delta_lon;
    
    (lat_rad.to_degrees(), lon_rad.to_degrees())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_distance_calculation() {
        // Test distance between two known points
        let lat1 = 40.7128; // New York
        let lon1 = -74.0060;
        let lat2 = 34.0522; // Los Angeles
        let lon2 = -118.2437;
        
        let distance = calculate_distance(lat1, lon1, lat2, lon2);
        
        // Distance should be approximately 3935 km
        assert!((distance - 3935000.0).abs() < 100000.0);
    }
    
    #[test]
    fn test_bearing_calculation() {
        let lat1 = 0.0;
        let lon1 = 0.0;
        let lat2 = 1.0;
        let lon2 = 0.0;
        
        let bearing = calculate_bearing(lat1, lon1, lat2, lon2);
        
        // Bearing should be approximately 0 degrees (north)
        assert!((bearing - 0.0).abs() < 1.0);
    }
    
    #[test]
    fn test_speed_conversions() {
        let speed_ms = 10.0;
        let speed_kmh = ms_to_kmh(speed_ms);
        let speed_mph = ms_to_mph(speed_ms);
        
        assert!((speed_kmh - 36.0).abs() < 0.1);
        assert!((speed_mph - 22.37).abs() < 0.1);
    }
    
    #[test]
    fn test_g_force_calculation() {
        let gx = 1.0;
        let gy = 1.0;
        let gz = 1.0;
        
        let magnitude = calculate_g_force_magnitude(gx, gy, gz);
        
        assert!((magnitude - 3.0.sqrt()).abs() < 0.001);
    }
} 