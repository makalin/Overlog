use image::{Rgba, RgbaImage, ImageBuffer};
// use rusttype::{Font, Scale};
use crate::{telemetry::TelemetryPoint, error::OverlogError};

pub struct OverlayRenderer {
    width: u32,
    height: u32,
    style: String,
    // font: Font<'static>,
}

impl OverlayRenderer {
    pub fn new(width: u32, height: u32, style: String) -> Result<Self, OverlogError> {
        // Load a default font (you might want to load from a file)
        // let font_data = include_bytes!("../assets/fonts/Roboto-Regular.ttf");
        // let font = Font::try_from_bytes(font_data as &[u8])
        //     .ok_or_else(|| OverlogError::Config("Failed to load font".to_string()))?;
        
        Ok(Self {
            width,
            height,
            style,
            // font,
        })
    }
    
    pub fn render_frame(&self, point: &TelemetryPoint, frame_number: u32) -> RgbaImage {
        let mut image = RgbaImage::new(self.width, self.height);
        
        // Clear with transparent background
        for pixel in image.pixels_mut() {
            *pixel = Rgba([0, 0, 0, 0]);
        }
        
        // Render speed display
        if let Some(speed) = point.speed {
            self.render_speed_display(&mut image, speed);
        }
        
        // Render g-force indicator
        if let (Some(gx), Some(gy), Some(gz)) = (point.g_force_x, point.g_force_y, point.g_force_z) {
            self.render_g_force_indicator(&mut image, gx, gy, gz);
        }
        
        // Render GPS coordinates
        if let (Some(lat), Some(lon)) = (point.latitude, point.longitude) {
            self.render_gps_display(&mut image, lat, lon);
        }
        
        // Render altitude
        if let Some(altitude) = point.altitude {
            self.render_altitude_display(&mut image, altitude);
        }
        
        // Render timestamp
        self.render_timestamp(&mut image, &point.timestamp);
        
        image
    }
    
    fn render_speed_display(&self, image: &mut RgbaImage, speed: f64) {
        let speed_kmh = crate::geo::ms_to_kmh(speed);
        let text = format!("{:.0} km/h", speed_kmh);
        
        // Simple text rendering without font
        self.draw_simple_text(image, &text, 50, 50, Rgba([255, 255, 255, 255]));
    }
    
    fn render_g_force_indicator(&self, image: &mut RgbaImage, gx: f64, gy: f64, gz: f64) {
        let magnitude = crate::geo::calculate_g_force_magnitude(gx, gy, gz);
        let text = format!("G: {:.2}", magnitude);
        
        let color = if magnitude > 2.0 {
            Rgba([255, 0, 0, 255]) // Red for high g-force
        } else {
            Rgba([255, 255, 255, 255])
        };
        
        self.draw_simple_text(image, &text, 50, 100, color);
        
        // Draw g-force ring
        self.draw_g_force_ring(image, gx, gy, gz);
    }
    
    fn render_gps_display(&self, image: &mut RgbaImage, lat: f64, lon: f64) {
        let text = format!("GPS: {:.6}, {:.6}", lat, lon);
        self.draw_simple_text(image, &text, 50, 150, Rgba([200, 200, 200, 255]));
    }
    
    fn render_altitude_display(&self, image: &mut RgbaImage, altitude: f64) {
        let text = format!("Alt: {:.0}m", altitude);
        self.draw_simple_text(image, &text, 50, 200, Rgba([255, 255, 255, 255]));
    }
    
    fn render_timestamp(&self, image: &mut RgbaImage, timestamp: &chrono::DateTime<chrono::Utc>) {
        let text = timestamp.format("%H:%M:%S").to_string();
        self.draw_simple_text(image, &text, self.width as i32 - 150, 50, Rgba([150, 150, 150, 255]));
    }
    
    fn draw_simple_text(&self, image: &mut RgbaImage, text: &str, x: i32, y: i32, color: Rgba<u8>) {
        // Simple 8x8 pixel font rendering
        let font_width = 8;
        let font_height = 8;
        let mut char_x = x;
        
        for ch in text.chars() {
            if char_x + font_width > image.width() as i32 {
                break;
            }
            
            // Simple character rendering (just draw a rectangle for now)
            for px in 0..font_width {
                for py in 0..font_height {
                    let pixel_x = char_x + px;
                    let pixel_y = y + py;
                    
                    if pixel_x >= 0 && pixel_x < image.width() as i32 && 
                       pixel_y >= 0 && pixel_y < image.height() as i32 {
                        image.put_pixel(pixel_x as u32, pixel_y as u32, color);
                    }
                }
            }
            
            char_x += font_width;
        }
    }
    
    fn draw_g_force_ring(&self, image: &mut RgbaImage, gx: f64, gy: f64, gz: f64) {
        let center_x = self.width as i32 / 2;
        let center_y = self.height as i32 / 2;
        let radius = 100;
        
        // Draw outer ring
        for angle in 0..360 {
            let rad = (angle as f64).to_radians();
            let x = center_x + (radius as f64 * rad.cos()) as i32;
            let y = center_y + (radius as f64 * rad.sin()) as i32;
            
            if x >= 0 && x < image.width() as i32 && y >= 0 && y < image.height() as i32 {
                image.put_pixel(x as u32, y as u32, Rgba([100, 100, 100, 255]));
            }
        }
        
        // Draw g-force vector
        let magnitude = crate::geo::calculate_g_force_magnitude(gx, gy, gz);
        let max_g = 3.0; // Maximum g-force for scaling
        let scaled_magnitude = (magnitude / max_g).min(1.0);
        
        let vector_x = center_x + (radius as f64 * scaled_magnitude * gx / magnitude) as i32;
        let vector_y = center_y + (radius as f64 * scaled_magnitude * gy / magnitude) as i32;
        
        if vector_x >= 0 && vector_x < image.width() as i32 && vector_y >= 0 && vector_y < image.height() as i32 {
            // Draw vector line
            self.draw_line(image, center_x, center_y, vector_x, vector_y, Rgba([255, 255, 0, 255]));
            
            // Draw vector endpoint
            for dx in -2..=2 {
                for dy in -2..=2 {
                    let x = vector_x + dx;
                    let y = vector_y + dy;
                    if x >= 0 && x < image.width() as i32 && y >= 0 && y < image.height() as i32 {
                        image.put_pixel(x as u32, y as u32, Rgba([255, 255, 0, 255]));
                    }
                }
            }
        }
    }
    
    fn draw_line(&self, image: &mut RgbaImage, x1: i32, y1: i32, x2: i32, y2: i32, color: Rgba<u8>) {
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;
        
        let mut x = x1;
        let mut y = y1;
        
        loop {
            if x >= 0 && x < image.width() as i32 && y >= 0 && y < image.height() as i32 {
                image.put_pixel(x as u32, y as u32, color);
            }
            
            if x == x2 && y == y2 {
                break;
            }
            
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }
} 