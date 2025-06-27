use overlog::telemetry::TelemetryData;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <telemetry_file>", args[0]);
        std::process::exit(1);
    }
    
    let file_path = &args[1];
    let content = std::fs::read_to_string(file_path)?;
    
    // Detect format from file extension
    let format = if file_path.ends_with(".gpx") {
        "gpx"
    } else if file_path.ends_with(".csv") {
        "csv"
    } else if file_path.ends_with(".json") {
        "json"
    } else {
        "unknown"
    };
    
    let telemetry = match format {
        "gpx" => TelemetryData::from_gpx(&content)?,
        "csv" => TelemetryData::from_csv(&content)?,
        "json" => TelemetryData::from_json(&content)?,
        _ => {
            eprintln!("Unsupported file format: {}", format);
            std::process::exit(1);
        }
    };
    
    println!("Successfully parsed {} telemetry points", telemetry.points.len());
    
    if let Some(duration) = telemetry.metadata.duration {
        println!("Duration: {:.2} seconds", duration);
    }
    
    if let Some(max_speed) = telemetry.metadata.max_speed {
        println!("Max speed: {:.1} km/h", max_speed * 3.6);
    }
    
    if let Some(total_distance) = telemetry.metadata.total_distance {
        println!("Total distance: {:.2} km", total_distance / 1000.0);
    }
    
    if let Some(max_g_force) = telemetry.metadata.max_g_force {
        println!("Max g-force: {:.2}g", max_g_force);
    }
    
    // Output as JSON
    let json_output = serde_json::to_string_pretty(&telemetry)?;
    println!("\nParsed data:");
    println!("{}", json_output);
    
    Ok(())
} 