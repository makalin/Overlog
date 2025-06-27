use std::fs;
use std::path::Path;
use crate::{telemetry::TelemetryData, error::OverlogError};

pub async fn parse_telemetry(
    input: String,
    output: Option<String>,
    format: Option<String>,
) -> Result<(), OverlogError> {
    let input_path = Path::new(&input);
    
    if !input_path.exists() {
        return Err(OverlogError::InvalidInput(format!("Input file not found: {}", input)));
    }
    
    let content = fs::read_to_string(input_path)?;
    let detected_format = format.unwrap_or_else(|| detect_format(input_path));
    
    let telemetry = match detected_format.as_str() {
        "gpx" => TelemetryData::from_gpx(&content)?,
        "csv" => TelemetryData::from_csv(&content)?,
        "json" => TelemetryData::from_json(&content)?,
        _ => return Err(OverlogError::UnsupportedFormat(detected_format)),
    };
    
    let json_output = serde_json::to_string_pretty(&telemetry)?;
    
    match output {
        Some(output_path) => {
            fs::write(output_path, json_output)?;
            println!("Telemetry data parsed and saved to output file");
        }
        None => {
            println!("{}", json_output);
        }
    }
    
    Ok(())
}

fn detect_format(path: &Path) -> String {
    if let Some(extension) = path.extension() {
        match extension.to_str().unwrap_or("").to_lowercase().as_str() {
            "gpx" => "gpx".to_string(),
            "csv" => "csv".to_string(),
            "json" => "json".to_string(),
            "tcx" => "tcx".to_string(),
            _ => "unknown".to_string(),
        }
    } else {
        "unknown".to_string()
    }
} 