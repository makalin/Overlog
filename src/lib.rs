pub mod commands;
pub mod error;
pub mod telemetry;
pub mod renderer;
pub mod video;
pub mod geo;
pub mod utils;

pub use error::OverlogError;
pub use telemetry::{TelemetryData, TelemetryPoint};
pub use renderer::OverlayRenderer;
pub use video::VideoProcessor;

/// Re-export commonly used types
pub mod prelude {
    pub use crate::{
        error::OverlogError,
        telemetry::{TelemetryData, TelemetryPoint},
        renderer::OverlayRenderer,
        video::VideoProcessor,
    };
} 