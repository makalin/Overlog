use overlog::{telemetry::TelemetryData, renderer::OverlayRenderer, video::VideoProcessor};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: {} <telemetry_file> <output_video>", args[0]);
        std::process::exit(1);
    }
    
    let telemetry_file = &args[1];
    let output_video = &args[2];
    
    println!("Loading telemetry data from: {}", telemetry_file);
    let content = std::fs::read_to_string(telemetry_file)?;
    let telemetry: TelemetryData = serde_json::from_str(&content)?;
    
    println!("Loaded {} telemetry points", telemetry.points.len());
    
    // Create renderer
    println!("Creating overlay renderer...");
    let renderer = OverlayRenderer::new(1920, 1080, "default".to_string())?;
    
    // Create video processor
    println!("Initializing video processor...");
    let processor = VideoProcessor::new()?;
    
    // Determine duration
    let duration = telemetry.metadata.duration.unwrap_or(30.0);
    println!("Rendering overlay for {:.2} seconds", duration);
    
    // Render overlay
    println!("Rendering overlay to: {}", output_video);
    processor.render_overlay(&renderer, &telemetry, output_video, 30, duration).await?;
    
    println!("Overlay rendering complete!");
    println!("Output file: {}", output_video);
    
    Ok(())
} 