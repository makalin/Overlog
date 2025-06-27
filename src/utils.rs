use std::path::Path;
use chrono::{DateTime, Utc};

/// Format a duration in seconds to a human-readable string
pub fn format_duration(seconds: f64) -> String {
    let hours = (seconds / 3600.0) as i32;
    let minutes = ((seconds % 3600.0) / 60.0) as i32;
    let secs = (seconds % 60.0) as i32;
    
    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{}:{:02}", minutes, secs)
    }
}

/// Format a speed value with appropriate units
pub fn format_speed(speed_ms: f64) -> String {
    let speed_kmh = speed_ms * 3.6;
    if speed_kmh >= 100.0 {
        format!("{:.0} km/h", speed_kmh)
    } else {
        format!("{:.1} km/h", speed_kmh)
    }
}

/// Format a distance value with appropriate units
pub fn format_distance(meters: f64) -> String {
    if meters >= 1000.0 {
        format!("{:.2} km", meters / 1000.0)
    } else {
        format!("{:.0} m", meters)
    }
}

/// Get file extension from path
pub fn get_file_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase())
}

/// Check if a file exists and is readable
pub fn file_exists_and_readable(path: &Path) -> bool {
    path.exists() && path.is_file() && std::fs::metadata(path).is_ok()
}

/// Create a timestamp string for filenames
pub fn create_timestamp_string() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.format("%Y%m%d_%H%M%S").to_string()
}

/// Validate video file format
pub fn is_valid_video_format(extension: &str) -> bool {
    matches!(extension, "mp4" | "mov" | "webm" | "avi" | "mkv" | "m4v")
}

/// Validate telemetry file format
pub fn is_valid_telemetry_format(extension: &str) -> bool {
    matches!(extension, "gpx" | "csv" | "json" | "tcx" | "bin")
}

/// Calculate frame number from timestamp
pub fn timestamp_to_frame(timestamp: DateTime<Utc>, start_time: DateTime<Utc>, fps: f64) -> u32 {
    let duration = timestamp.signed_duration_since(start_time);
    let seconds = duration.num_milliseconds() as f64 / 1000.0;
    (seconds * fps) as u32
}

/// Calculate timestamp from frame number
pub fn frame_to_timestamp(frame: u32, start_time: DateTime<Utc>, fps: f64) -> DateTime<Utc> {
    let seconds = frame as f64 / fps;
    start_time + chrono::Duration::milliseconds((seconds * 1000.0) as i64)
}

/// Clamp a value between min and max
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Linear interpolation between two values
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

/// Convert degrees to radians
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

/// Convert radians to degrees
pub fn radians_to_degrees(radians: f64) -> f64 {
    radians * 180.0 / std::f64::consts::PI
}

/// Normalize an angle to 0-360 degrees
pub fn normalize_angle(angle: f64) -> f64 {
    let mut normalized = angle % 360.0;
    if normalized < 0.0 {
        normalized += 360.0;
    }
    normalized
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(65.0), "1:05");
        assert_eq!(format_duration(3661.0), "1:01:01");
        assert_eq!(format_duration(30.0), "0:30");
    }

    #[test]
    fn test_format_speed() {
        assert_eq!(format_speed(10.0), "36.0 km/h");
        assert_eq!(format_speed(27.78), "100.0 km/h");
    }

    #[test]
    fn test_format_distance() {
        assert_eq!(format_distance(500.0), "500 m");
        assert_eq!(format_distance(1500.0), "1.50 km");
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(5, 0, 10), 5);
        assert_eq!(clamp(-5, 0, 10), 0);
        assert_eq!(clamp(15, 0, 10), 10);
    }

    #[test]
    fn test_lerp() {
        assert_eq!(lerp(0.0, 10.0, 0.5), 5.0);
        assert_eq!(lerp(0.0, 10.0, 0.0), 0.0);
        assert_eq!(lerp(0.0, 10.0, 1.0), 10.0);
    }

    #[test]
    fn test_angle_conversions() {
        assert_eq!(degrees_to_radians(180.0), std::f64::consts::PI);
        assert_eq!(radians_to_degrees(std::f64::consts::PI), 180.0);
    }

    #[test]
    fn test_normalize_angle() {
        assert_eq!(normalize_angle(370.0), 10.0);
        assert_eq!(normalize_angle(-10.0), 350.0);
        assert_eq!(normalize_angle(360.0), 0.0);
    }

    #[test]
    fn test_timestamp_conversions() {
        let start_time = Utc::now();
        let fps = 30.0;
        
        let frame = timestamp_to_frame(start_time + Duration::seconds(1), start_time, fps);
        assert_eq!(frame, 30);
        
        let timestamp = frame_to_timestamp(30, start_time, fps);
        let diff = timestamp.signed_duration_since(start_time).num_milliseconds();
        assert!((diff - 1000).abs() < 50); // Allow small rounding errors
    }
} 