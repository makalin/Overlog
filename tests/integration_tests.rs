use overlog::{
    telemetry::TelemetryData,
    renderer::OverlayRenderer,
    video::VideoProcessor,
    error::OverlogError,
};
use tempfile::TempDir;
use std::fs;

#[tokio::test]
async fn test_telemetry_parsing() -> Result<(), OverlogError> {
    let json_data = r#"{
        "points": [
            {
                "timestamp": "2024-01-15T10:00:00Z",
                "latitude": 40.7128,
                "longitude": -74.0060,
                "altitude": 10.0,
                "speed": 8.33
            }
        ],
        "metadata": {
            "source": "test",
            "format": "json"
        }
    }"#;
    
    let telemetry = TelemetryData::from_json(json_data)?;
    
    assert_eq!(telemetry.points.len(), 1);
    assert_eq!(telemetry.points[0].speed, Some(8.33));
    assert_eq!(telemetry.points[0].latitude, Some(40.7128));
    
    Ok(())
}

#[tokio::test]
async fn test_gpx_parsing() -> Result<(), OverlogError> {
    let gpx_data = r#"<?xml version="1.0" encoding="UTF-8"?>
    <gpx version="1.1" creator="Test" xmlns="http://www.topografix.com/GPX/1/1">
        <trk>
            <trkseg>
                <trkpt lat="40.7128" lon="-74.0060">
                    <ele>10.0</ele>
                    <time>2024-01-15T10:00:00Z</time>
                </trkpt>
            </trkseg>
        </trk>
    </gpx>"#;
    
    let telemetry = TelemetryData::from_gpx(gpx_data)?;
    
    assert_eq!(telemetry.points.len(), 1);
    assert_eq!(telemetry.points[0].latitude, Some(40.7128));
    assert_eq!(telemetry.points[0].longitude, Some(-74.0060));
    assert_eq!(telemetry.points[0].altitude, Some(10.0));
    
    Ok(())
}

#[tokio::test]
async fn test_csv_parsing() -> Result<(), OverlogError> {
    let csv_data = "timestamp,latitude,longitude,altitude,speed\n2024-01-15T10:00:00Z,40.7128,-74.0060,10.0,8.33";
    
    let telemetry = TelemetryData::from_csv(csv_data)?;
    
    assert_eq!(telemetry.points.len(), 1);
    assert_eq!(telemetry.points[0].speed, Some(8.33));
    
    Ok(())
}

#[tokio::test]
async fn test_renderer_creation() -> Result<(), OverlogError> {
    let renderer = OverlayRenderer::new(1920, 1080, "default".to_string())?;
    
    // Test that renderer was created successfully
    assert_eq!(renderer.width, 1920);
    assert_eq!(renderer.height, 1080);
    
    Ok(())
}

#[tokio::test]
async fn test_frame_rendering() -> Result<(), OverlogError> {
    let renderer = OverlayRenderer::new(640, 480, "default".to_string())?;
    
    let point = overlog::telemetry::TelemetryPoint {
        timestamp: chrono::Utc::now(),
        latitude: Some(40.7128),
        longitude: Some(-74.0060),
        altitude: Some(10.0),
        speed: Some(8.33),
        heading: Some(90.0),
        g_force_x: Some(0.1),
        g_force_y: Some(0.0),
        g_force_z: Some(1.0),
        acceleration: Some(0.5),
        rpm: Some(2000),
        throttle: Some(0.3),
        brake: Some(0.0),
        steering: Some(0.1),
    };
    
    let frame = renderer.render_frame(&point, 0);
    
    assert_eq!(frame.width(), 640);
    assert_eq!(frame.height(), 480);
    
    Ok(())
}

#[tokio::test]
async fn test_telemetry_interpolation() -> Result<(), OverlogError> {
    let mut telemetry = TelemetryData::new();
    telemetry.metadata.format = "test".to_string();
    
    let point1 = overlog::telemetry::TelemetryPoint {
        timestamp: chrono::Utc::now(),
        latitude: Some(40.7128),
        longitude: Some(-74.0060),
        speed: Some(8.0),
        ..Default::default()
    };
    
    let point2 = overlog::telemetry::TelemetryPoint {
        timestamp: point1.timestamp + chrono::Duration::seconds(1),
        latitude: Some(40.7129),
        longitude: Some(-74.0059),
        speed: Some(10.0),
        ..Default::default()
    };
    
    telemetry.points.push(point1.clone());
    telemetry.points.push(point2);
    telemetry.calculate_metadata();
    
    let interpolated = telemetry.interpolate_at_time(
        point1.timestamp + chrono::Duration::milliseconds(500)
    );
    
    assert!(interpolated.is_some());
    let point = interpolated.unwrap();
    assert!(point.speed.unwrap() > 8.0 && point.speed.unwrap() < 10.0);
    
    Ok(())
}

#[tokio::test]
async fn test_geo_calculations() {
    let distance = overlog::geo::calculate_distance(40.7128, -74.0060, 40.7129, -74.0059);
    assert!(distance > 0.0);
    
    let bearing = overlog::geo::calculate_bearing(40.7128, -74.0060, 40.7129, -74.0059);
    assert!(bearing >= 0.0 && bearing <= 360.0);
    
    let speed_kmh = overlog::geo::ms_to_kmh(10.0);
    assert_eq!(speed_kmh, 36.0);
    
    let g_force = overlog::geo::calculate_g_force_magnitude(1.0, 1.0, 1.0);
    assert_eq!(g_force, 3.0_f64.sqrt());
}

#[tokio::test]
async fn test_utils_functions() {
    let duration = overlog::utils::format_duration(65.0);
    assert_eq!(duration, "1:05");
    
    let speed = overlog::utils::format_speed(10.0);
    assert_eq!(speed, "36.0 km/h");
    
    let distance = overlog::utils::format_distance(1500.0);
    assert_eq!(distance, "1.50 km");
    
    let clamped = overlog::utils::clamp(15, 0, 10);
    assert_eq!(clamped, 10);
    
    let lerped = overlog::utils::lerp(0.0, 10.0, 0.5);
    assert_eq!(lerped, 5.0);
} 