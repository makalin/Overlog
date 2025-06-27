use std::fs;
use crate::{telemetry::TelemetryData, renderer::OverlayRenderer, video::VideoProcessor, error::OverlogError};

pub async fn render_overlay(
    input: String,
    output: String,
    width: u32,
    height: u32,
    duration: Option<f64>,
    fps: u32,
    style: String,
) -> Result<(), OverlogError> {
    // Load telemetry data
    let content = fs::read_to_string(&input)?;
    let telemetry: TelemetryData = serde_json::from_str(&content)?;
    
    // Create renderer
    let renderer = OverlayRenderer::new(width, height, style)?;
    
    // Determine duration
    let video_duration = duration.unwrap_or_else(|| {
        telemetry.metadata.duration.unwrap_or(30.0)
    });
    
    // Create video processor
    let processor = VideoProcessor::new()?;
    
    // Render overlay
    processor.render_overlay(&renderer, &telemetry, &output, fps, video_duration).await?;
    
    println!("Overlay rendered to: {}", output);
    Ok(())
}

pub async fn burn_overlay(
    video: String,
    overlay: String,
    output: String,
    offset: f64,
) -> Result<(), OverlogError> {
    // Create video processor
    let processor = VideoProcessor::new()?;
    
    // Burn overlay into video
    processor.burn_overlay(&video, &overlay, &output, offset).await?;
    
    println!("Overlay burned into video: {}", output);
    Ok(())
} 