use std::process::Command;
use std::path::Path;
use crate::{telemetry::TelemetryData, renderer::OverlayRenderer, error::OverlogError};

pub struct VideoProcessor;

impl VideoProcessor {
    pub fn new() -> Result<Self, OverlogError> {
        // Check if FFmpeg is available
        let output = Command::new("ffmpeg")
            .arg("-version")
            .output()
            .map_err(|_| OverlogError::Config("FFmpeg not found. Please install FFmpeg and ensure it's in your PATH.".to_string()))?;
        
        if !output.status.success() {
            return Err(OverlogError::Config("FFmpeg is not working properly".to_string()));
        }
        
        Ok(Self)
    }
    
    pub async fn render_overlay(
        &self,
        renderer: &OverlayRenderer,
        telemetry: &TelemetryData,
        output_path: &str,
        fps: u32,
        duration: f64,
    ) -> Result<(), OverlogError> {
        let temp_dir = std::env::temp_dir().join("overlog_frames");
        std::fs::create_dir_all(&temp_dir)?;
        
        let total_frames = (duration * fps as f64) as u32;
        let frame_duration = duration / total_frames as f64;
        
        // Generate frames
        for frame_num in 0..total_frames {
            let timestamp = if let Some(start_time) = telemetry.metadata.start_time {
                start_time + chrono::Duration::milliseconds((frame_num as f64 * frame_duration * 1000.0) as i64)
            } else {
                chrono::Utc::now()
            };
            
            let point = telemetry.interpolate_at_time(timestamp)
                .unwrap_or_else(|| telemetry.points.first().cloned().unwrap_or_default());
            
            let frame = renderer.render_frame(&point, frame_num);
            let frame_path = temp_dir.join(format!("frame_{:06}.png", frame_num));
            frame.save(&frame_path)?;
        }
        
        // Create video from frames using FFmpeg
        let frame_pattern = temp_dir.join("frame_%06d.png").to_string_lossy().to_string();
        
        let status = Command::new("ffmpeg")
            .args(&[
                "-y", // Overwrite output
                "-framerate", &fps.to_string(),
                "-i", &frame_pattern,
                "-c:v", "libvpx-vp9",
                "-pix_fmt", "yuva420p", // Support alpha channel
                "-crf", "30",
                "-b:v", "0",
                output_path,
            ])
            .status()?;
        
        if !status.success() {
            return Err(OverlogError::Ffmpeg("Failed to create video from frames".to_string()));
        }
        
        // Clean up temporary files
        for entry in std::fs::read_dir(&temp_dir)? {
            if let Ok(entry) = entry {
                let _ = std::fs::remove_file(entry.path());
            }
        }
        let _ = std::fs::remove_dir(&temp_dir);
        
        Ok(())
    }
    
    pub async fn burn_overlay(
        &self,
        video_path: &str,
        overlay_path: &str,
        output_path: &str,
        offset: f64,
    ) -> Result<(), OverlogError> {
        if !Path::new(video_path).exists() {
            return Err(OverlogError::InvalidInput(format!("Video file not found: {}", video_path)));
        }
        
        if !Path::new(overlay_path).exists() {
            return Err(OverlogError::InvalidInput(format!("Overlay file not found: {}", overlay_path)));
        }
        
        let offset_arg = if offset != 0.0 {
            format!(":enable='between(t,{},{})'", offset, offset + 999999.0)
        } else {
            String::new()
        };
        
        let filter_complex = format!(
            "[0:v][1:v]overlay=0:0{}[outv]",
            offset_arg
        );
        
        let status = Command::new("ffmpeg")
            .args(&[
                "-y", // Overwrite output
                "-i", video_path,
                "-i", overlay_path,
                "-filter_complex", &filter_complex,
                "-map", "[outv]",
                "-map", "0:a", // Copy audio from original video
                "-c:a", "copy",
                output_path,
            ])
            .status()?;
        
        if !status.success() {
            return Err(OverlogError::Ffmpeg("Failed to burn overlay into video".to_string()));
        }
        
        Ok(())
    }
    
    pub fn get_video_info(&self, video_path: &str) -> Result<VideoInfo, OverlogError> {
        let output = Command::new("ffprobe")
            .args(&[
                "-v", "quiet",
                "-print_format", "json",
                "-show_format",
                "-show_streams",
                video_path,
            ])
            .output()?;
        
        if !output.status.success() {
            return Err(OverlogError::Ffmpeg("Failed to get video info".to_string()));
        }
        
        let info: serde_json::Value = serde_json::from_slice(&output.stdout)?;
        
        let duration = info["format"]["duration"]
            .as_str()
            .and_then(|s| s.parse::<f64>().ok());
        
        let width = info["streams"][0]["width"]
            .as_u64()
            .unwrap_or(0) as u32;
        
        let height = info["streams"][0]["height"]
            .as_u64()
            .unwrap_or(0) as u32;
        
        let fps_str = info["streams"][0]["r_frame_rate"]
            .as_str()
            .unwrap_or("30/1");
        
        let fps = parse_fps(fps_str);
        
        Ok(VideoInfo {
            duration,
            width,
            height,
            fps,
        })
    }
}

fn parse_fps(fps_str: &str) -> f64 {
    let parts: Vec<&str> = fps_str.split('/').collect();
    if parts.len() == 2 {
        if let (Ok(num), Ok(den)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
            if den != 0.0 {
                return num / den;
            }
        }
    }
    30.0 // Default fallback
}

#[derive(Debug, Clone)]
pub struct VideoInfo {
    pub duration: Option<f64>,
    pub width: u32,
    pub height: u32,
    pub fps: f64,
} 