use thiserror::Error;

#[derive(Error, Debug)]
pub enum OverlogError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("CSV parsing error: {0}")]
    Csv(#[from] csv::Error),
    
    #[error("GPX parsing error: {0}")]
    Gpx(#[from] gpx::errors::GpxError),
    
    #[error("FFmpeg error: {0}")]
    Ffmpeg(String),
    
    #[error("Video processing error: {0}")]
    Video(String),
    
    #[error("Telemetry parsing error: {0}")]
    Telemetry(String),
    
    #[error("Geographic calculation error: {0}")]
    Geo(String),
    
    #[error("Rendering error: {0}")]
    Rendering(String),
    
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Image processing error: {0}")]
    Image(#[from] image::ImageError),
}

impl From<anyhow::Error> for OverlogError {
    fn from(err: anyhow::Error) -> Self {
        OverlogError::InvalidInput(err.to_string())
    }
} 