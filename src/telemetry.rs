use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::error::OverlogError;
use std::io::Cursor;
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TelemetryPoint {
    pub timestamp: DateTime<Utc>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub altitude: Option<f64>,
    pub speed: Option<f64>,
    pub heading: Option<f64>,
    pub g_force_x: Option<f64>,
    pub g_force_y: Option<f64>,
    pub g_force_z: Option<f64>,
    pub acceleration: Option<f64>,
    pub rpm: Option<f64>,
    pub throttle: Option<f64>,
    pub brake: Option<f64>,
    pub steering: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryData {
    pub points: Vec<TelemetryPoint>,
    pub metadata: TelemetryMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryMetadata {
    pub source: String,
    pub format: String,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration: Option<f64>,
    pub total_distance: Option<f64>,
    pub max_speed: Option<f64>,
    pub max_g_force: Option<f64>,
}

impl TelemetryData {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            metadata: TelemetryMetadata {
                source: String::new(),
                format: String::new(),
                start_time: None,
                end_time: None,
                duration: None,
                total_distance: None,
                max_speed: None,
                max_g_force: None,
            },
        }
    }
    
    pub fn from_gpx(gpx_data: &str) -> Result<Self, OverlogError> {
        let cursor = Cursor::new(gpx_data.as_bytes());
        let gpx = gpx::read(cursor)?;
        let mut telemetry = TelemetryData::new();
        telemetry.metadata.format = "gpx".to_string();
        
        for track in gpx.tracks {
            for segment in track.segments {
                for point in segment.points {
                    let timestamp = if let Some(time) = point.time {
                        // Convert gpx::Time (which wraps time::OffsetDateTime) to chrono::DateTime<Utc>
                        let offset_datetime: OffsetDateTime = time.into();
                        let unix_timestamp = offset_datetime.unix_timestamp();
                        let naive = chrono::DateTime::from_timestamp(unix_timestamp, 0)
                            .unwrap_or_else(|| chrono::Utc::now())
                            .naive_utc();
                        chrono::DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc)
                    } else {
                        Utc::now()
                    };
                    
                    let mut tp = TelemetryPoint {
                        timestamp,
                        latitude: Some(point.point().y()),
                        longitude: Some(point.point().x()),
                        altitude: point.elevation,
                        speed: None,
                        heading: None,
                        g_force_x: None,
                        g_force_y: None,
                        g_force_z: None,
                        acceleration: None,
                        rpm: None,
                        throttle: None,
                        brake: None,
                        steering: None,
                    };
                    
                    // Extract speed from extensions if available
                    // Note: The new gpx API may have different extension handling
                    // This is a simplified version - you may need to adjust based on actual GPX structure
                    
                    telemetry.points.push(tp);
                }
            }
        }
        
        telemetry.calculate_metadata();
        Ok(telemetry)
    }
    
    pub fn from_csv(csv_data: &str) -> Result<Self, OverlogError> {
        let mut telemetry = TelemetryData::new();
        telemetry.metadata.format = "csv".to_string();
        
        let mut reader = csv::Reader::from_reader(csv_data.as_bytes());
        
        for result in reader.deserialize() {
            let point: TelemetryPoint = result?;
            telemetry.points.push(point);
        }
        
        telemetry.calculate_metadata();
        Ok(telemetry)
    }
    
    pub fn from_json(json_data: &str) -> Result<Self, OverlogError> {
        let telemetry: TelemetryData = serde_json::from_str(json_data)?;
        Ok(telemetry)
    }
    
    pub fn calculate_metadata(&mut self) {
        if self.points.is_empty() {
            return;
        }
        
        self.points.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        
        self.metadata.start_time = Some(self.points.first().unwrap().timestamp);
        self.metadata.end_time = Some(self.points.last().unwrap().timestamp);
        
        if let (Some(start), Some(end)) = (self.metadata.start_time, self.metadata.end_time) {
            self.metadata.duration = Some((end - start).num_seconds() as f64);
        }
        
        // Calculate max speed
        self.metadata.max_speed = self.points
            .iter()
            .filter_map(|p| p.speed)
            .max_by(|a, b| a.partial_cmp(b).unwrap());
        
        // Calculate max g-force
        let max_g = self.points
            .iter()
            .filter_map(|p| {
                if let (Some(x), Some(y), Some(z)) = (p.g_force_x, p.g_force_y, p.g_force_z) {
                    Some((x * x + y * y + z * z).sqrt())
                } else {
                    None
                }
            })
            .max_by(|a, b| a.partial_cmp(b).unwrap());
        
        self.metadata.max_g_force = max_g;
        
        // Calculate total distance
        self.metadata.total_distance = self.calculate_total_distance();
    }
    
    fn calculate_total_distance(&self) -> Option<f64> {
        if self.points.len() < 2 {
            return None;
        }
        
        let mut total_distance = 0.0;
        
        for window in self.points.windows(2) {
            if let (Some(lat1), Some(lon1), Some(lat2), Some(lon2)) = 
                (window[0].latitude, window[0].longitude, window[1].latitude, window[1].longitude) {
                
                let distance = crate::geo::calculate_distance(lat1, lon1, lat2, lon2);
                total_distance += distance;
            }
        }
        
        Some(total_distance)
    }
    
    pub fn get_point_at_time(&self, timestamp: DateTime<Utc>) -> Option<&TelemetryPoint> {
        self.points.binary_search_by(|point| point.timestamp.cmp(&timestamp))
            .ok()
            .map(|index| &self.points[index])
    }
    
    pub fn interpolate_at_time(&self, timestamp: DateTime<Utc>) -> Option<TelemetryPoint> {
        // Find the two points that bracket the timestamp
        let index = self.points.binary_search_by(|point| point.timestamp.cmp(&timestamp));
        
        match index {
            Ok(i) => Some(self.points[i].clone()),
            Err(i) => {
                if i == 0 || i >= self.points.len() {
                    None
                } else {
                    // Interpolate between points[i-1] and points[i]
                    let p1 = &self.points[i - 1];
                    let p2 = &self.points[i];
                    
                    let t1 = p1.timestamp.timestamp_millis() as f64;
                    let t2 = p2.timestamp.timestamp_millis() as f64;
                    let t = timestamp.timestamp_millis() as f64;
                    
                    let ratio = (t - t1) / (t2 - t1);
                    
                    Some(TelemetryPoint {
                        timestamp,
                        latitude: interpolate_option(p1.latitude, p2.latitude, ratio),
                        longitude: interpolate_option(p1.longitude, p2.longitude, ratio),
                        altitude: interpolate_option(p1.altitude, p2.altitude, ratio),
                        speed: interpolate_option(p1.speed, p2.speed, ratio),
                        heading: interpolate_option(p1.heading, p2.heading, ratio),
                        g_force_x: interpolate_option(p1.g_force_x, p2.g_force_x, ratio),
                        g_force_y: interpolate_option(p1.g_force_y, p2.g_force_y, ratio),
                        g_force_z: interpolate_option(p1.g_force_z, p2.g_force_z, ratio),
                        acceleration: interpolate_option(p1.acceleration, p2.acceleration, ratio),
                        rpm: interpolate_option(p1.rpm, p2.rpm, ratio),
                        throttle: interpolate_option(p1.throttle, p2.throttle, ratio),
                        brake: interpolate_option(p1.brake, p2.brake, ratio),
                        steering: interpolate_option(p1.steering, p2.steering, ratio),
                    })
                }
            }
        }
    }
}

fn interpolate_option(a: Option<f64>, b: Option<f64>, ratio: f64) -> Option<f64> {
    match (a, b) {
        (Some(a_val), Some(b_val)) => Some(a_val + (b_val - a_val) * ratio),
        (Some(a_val), None) => Some(a_val),
        (None, Some(b_val)) => Some(b_val),
        (None, None) => None,
    }
} 