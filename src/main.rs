use clap::{Parser, Subcommand};
use overlog::{
    commands::{parse, render},
    error::OverlogError,
};

#[derive(Parser)]
#[command(name = "overlog")]
#[command(about = "Terminal-based tool for overlaying telemetry data onto video files")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse telemetry data from various formats
    Parse {
        /// Input file path
        #[arg(short, long)]
        input: String,
        
        /// Output file path (optional, defaults to stdout)
        #[arg(short, long)]
        output: Option<String>,
        
        /// Input format (auto-detected if not specified)
        #[arg(short, long)]
        format: Option<String>,
    },
    
    /// Render telemetry overlay
    Render {
        /// Input telemetry data file
        #[arg(short, long)]
        input: String,
        
        /// Output video file
        #[arg(short, long)]
        output: String,
        
        /// Video width
        #[arg(long, default_value = "1920")]
        width: u32,
        
        /// Video height
        #[arg(long, default_value = "1080")]
        height: u32,
        
        /// Video duration in seconds
        #[arg(long)]
        duration: Option<f64>,
        
        /// Frame rate
        #[arg(long, default_value = "30")]
        fps: u32,
        
        /// Overlay style
        #[arg(long, default_value = "default")]
        style: String,
    },
    
    /// Burn overlay into video file
    Burn {
        /// Input video file
        #[arg(short, long)]
        video: String,
        
        /// Input overlay file
        #[arg(short, long)]
        overlay: String,
        
        /// Output video file
        #[arg(short, long)]
        output: String,
        
        /// Sync offset in seconds
        #[arg(long, default_value = "0.0")]
        offset: f64,
    },
}

#[tokio::main]
async fn main() -> Result<(), OverlogError> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Parse { input, output, format } => {
            parse::parse_telemetry(input, output, format).await?;
        }
        Commands::Render { input, output, width, height, duration, fps, style } => {
            render::render_overlay(input, output, width, height, duration, fps, style).await?;
        }
        Commands::Burn { video, overlay, output, offset } => {
            render::burn_overlay(video, overlay, output, offset).await?;
        }
    }
    
    Ok(())
} 