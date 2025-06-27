# Overlog API Documentation

## Overview

Overlog is a Rust library for processing telemetry data and creating video overlays. This document describes the public API.

## Core Types

### TelemetryData

The main data structure containing telemetry points and metadata.

```rust
pub struct TelemetryData {
    pub points: Vec<TelemetryPoint>,
    pub metadata: TelemetryMetadata,
}
```

#### Methods

- `new() -> Self` - Create a new empty TelemetryData instance
- `from_gpx(data: &str) -> Result<Self, OverlogError>` - Parse GPX data
- `from_csv(data: &str) -> Result<Self, OverlogError>` - Parse CSV data
- `from_json(data: &str) -> Result<Self, OverlogError>` - Parse JSON data
- `calculate_metadata(&mut self)` - Calculate metadata from points
- `get_point_at_time(timestamp: DateTime<Utc>) -> Option<&TelemetryPoint>` - Get point at specific time
- `interpolate_at_time(timestamp: DateTime<Utc>) -> Option<TelemetryPoint>` - Interpolate point at time

### TelemetryPoint

Represents a single telemetry data point.

```rust
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
```

### OverlayRenderer

Handles rendering of telemetry overlays to images.

```rust
pub struct OverlayRenderer {
    width: u32,
    height: u32,
    style: String,
    font: Font<'static>,
}
```

#### Methods

- `new(width: u32, height: u32, style: String) -> Result<Self, OverlogError>` - Create new renderer
- `render_frame(point: &TelemetryPoint, frame_number: u32) -> RgbaImage` - Render single frame

### VideoProcessor

Handles video processing operations using FFmpeg.

```rust
pub struct VideoProcessor;
```

#### Methods

- `new() -> Result<Self, OverlogError>` - Create new processor
- `render_overlay(renderer: &OverlayRenderer, telemetry: &TelemetryData, output: &str, fps: u32, duration: f64) -> Result<(), OverlogError>` - Render overlay video
- `burn_overlay(video: &str, overlay: &str, output: &str, offset: f64) -> Result<(), OverlogError>` - Burn overlay into video
- `get_video_info(video_path: &str) -> Result<VideoInfo, OverlogError>` - Get video information

## Geographic Functions

### Distance and Bearing

- `calculate_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64` - Calculate distance between points
- `calculate_bearing(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64` - Calculate bearing between points
- `calculate_destination(lat: f64, lon: f64, bearing: f64, distance: f64) -> (f64, f64)` - Calculate destination point

### Speed Conversions

- `ms_to_kmh(speed_ms: f64) -> f64` - Convert m/s to km/h
- `kmh_to_ms(speed_kmh: f64) -> f64` - Convert km/h to m/s
- `ms_to_mph(speed_ms: f64) -> f64` - Convert m/s to mph
- `mph_to_ms(speed_mph: f64) -> f64` - Convert mph to m/s

### G-Force Calculations

- `calculate_g_force_magnitude(gx: f64, gy: f64, gz: f64) -> f64` - Calculate total g-force magnitude
- `calculate_acceleration(speed1: f64, speed2: f64, time_delta: f64) -> f64` - Calculate acceleration

## Utility Functions

### Formatting

- `format_duration(seconds: f64) -> String` - Format duration as HH:MM:SS
- `format_speed(speed_ms: f64) -> String` - Format speed with units
- `format_distance(meters: f64) -> String` - Format distance with units

### File Operations

- `get_file_extension(path: &Path) -> Option<String>` - Get file extension
- `file_exists_and_readable(path: &Path) -> bool` - Check if file exists and is readable
- `is_valid_video_format(extension: &str) -> bool` - Validate video format
- `is_valid_telemetry_format(extension: &str) -> bool` - Validate telemetry format

### Time and Frame Conversions

- `timestamp_to_frame(timestamp: DateTime<Utc>, start_time: DateTime<Utc>, fps: f64) -> u32` - Convert timestamp to frame
- `frame_to_timestamp(frame: u32, start_time: DateTime<Utc>, fps: f64) -> DateTime<Utc>` - Convert frame to timestamp

### Mathematical Utilities

- `clamp<T: PartialOrd>(value: T, min: T, max: T) -> T` - Clamp value between min and max
- `lerp(a: f64, b: f64, t: f64) -> f64` - Linear interpolation
- `degrees_to_radians(degrees: f64) -> f64` - Convert degrees to radians
- `radians_to_degrees(radians: f64) -> f64` - Convert radians to degrees
- `normalize_angle(angle: f64) -> f64` - Normalize angle to 0-360 degrees

## Error Handling

All functions that can fail return `Result<T, OverlogError>`. The `OverlogError` enum includes:

- `Io(std::io::Error)` - IO errors
- `Json(serde_json::Error)` - JSON serialization errors
- `Csv(csv::Error)` - CSV parsing errors
- `Gpx(gpx::Error)` - GPX parsing errors
- `Ffmpeg(String)` - FFmpeg-related errors
- `Video(String)` - Video processing errors
- `Telemetry(String)` - Telemetry parsing errors
- `Geo(String)` - Geographic calculation errors
- `Rendering(String)` - Rendering errors
- `UnsupportedFormat(String)` - Unsupported file format
- `InvalidInput(String)` - Invalid input data
- `Config(String)` - Configuration errors

## Examples

### Basic Usage

```rust
use overlog::{TelemetryData, OverlayRenderer, VideoProcessor};

// Parse telemetry data
let telemetry = TelemetryData::from_json(json_data)?;

// Create renderer
let renderer = OverlayRenderer::new(1920, 1080, "default".to_string())?;

// Create video processor
let processor = VideoProcessor::new()?;

// Render overlay
processor.render_overlay(&renderer, &telemetry, "output.webm", 30, 60.0).await?;
```

### Geographic Calculations

```rust
use overlog::geo;

let distance = geo::calculate_distance(40.7128, -74.0060, 40.7129, -74.0059);
let speed_kmh = geo::ms_to_kmh(10.0);
let g_force = geo::calculate_g_force_magnitude(1.0, 1.0, 1.0);
```

### Utility Functions

```rust
use overlog::utils;

let duration = utils::format_duration(3661.0); // "1:01:01"
let speed = utils::format_speed(10.0); // "36.0 km/h"
let distance = utils::format_distance(1500.0); // "1.50 km"
``` 