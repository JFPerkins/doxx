use anyhow::Result;
use std::path::Path;

/// Terminal image display capabilities
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TerminalImageSupport {
    Kitty,      // Kitty graphics protocol
    ITerm2,     // iTerm2 graphics protocol
    Sixel,      // Sixel graphics
    HalfBlocks, // Unicode half-block fallback
    None,       // Text description only
}

/// Handles display of images in the terminal using various protocols
#[derive(Debug)]
pub struct TerminalImageRenderer {
    support: TerminalImageSupport,
    max_width: u32,
    max_height: u32,
}

impl TerminalImageRenderer {
    /// Create a new terminal image renderer with auto-detected capabilities
    pub fn new() -> Self {
        let support = Self::detect_capabilities();
        let (max_width, max_height) = Self::get_terminal_size();

        Self {
            support,
            max_width,
            max_height,
        }
    }

    /// Create a new terminal image renderer with custom size limits
    pub fn with_size_limits(max_width: Option<u32>, max_height: Option<u32>) -> Self {
        let support = Self::detect_capabilities();
        let (default_width, default_height) = Self::get_terminal_size();

        Self {
            support,
            max_width: max_width.unwrap_or(default_width),
            max_height: max_height.unwrap_or(default_height),
        }
    }

    /// Create a new terminal image renderer with custom size limits and scaling
    pub fn with_options(
        max_width: Option<u32>,
        max_height: Option<u32>,
        scale: Option<f32>,
    ) -> Self {
        let support = Self::detect_capabilities();
        let (default_width, default_height) = Self::get_terminal_size();

        let scale_factor = scale.unwrap_or(1.0).clamp(0.1, 2.0); // Clamp between 0.1 and 2.0

        let scaled_width = max_width.unwrap_or(default_width);
        let scaled_height = max_height.unwrap_or(default_height);

        Self {
            support,
            max_width: ((scaled_width as f32) * scale_factor) as u32,
            max_height: ((scaled_height as f32) * scale_factor) as u32,
        }
    }

    /// Create a renderer with specific capabilities (for testing)
    pub fn with_support(support: TerminalImageSupport) -> Self {
        let (max_width, max_height) = Self::get_terminal_size();

        Self {
            support,
            max_width,
            max_height,
        }
    }

    /// Detect terminal image display capabilities
    pub fn detect_capabilities() -> TerminalImageSupport {
        // Check for WezTerm FIRST - it supports Kitty protocol
        if let Ok(term_program) = std::env::var("TERM_PROGRAM") {
            if term_program == "WezTerm" {
                return TerminalImageSupport::Kitty;
            }
        }

        // Check for iTerm2 (this function exists)
        if viuer::is_iterm_supported() {
            return TerminalImageSupport::ITerm2;
        }

        // Sixel support disabled for now to avoid linking issues
        // Will re-enable after fixing dependencies

        // Check terminal type for Kitty support
        if let Ok(term) = std::env::var("TERM") {
            match term.as_str() {
                "xterm-kitty" => TerminalImageSupport::Kitty,
                "wezterm" => TerminalImageSupport::Kitty,
                "screen" | "screen-256color" => {
                    // Screen/tmux might support passthrough
                    TerminalImageSupport::HalfBlocks
                }
                _ => TerminalImageSupport::HalfBlocks,
            }
        } else {
            TerminalImageSupport::HalfBlocks
        }
    }

    /// Get the current support level
    pub fn support(&self) -> TerminalImageSupport {
        self.support
    }

    /// Check if we can display images inline
    pub fn can_display_images(&self) -> bool {
        !matches!(self.support, TerminalImageSupport::None)
    }

    /// Render an image from a file path
    pub fn render_image_from_path(&self, image_path: &Path, description: &str) -> Result<()> {
        match self.support {
            TerminalImageSupport::None => {
                println!("📷 Image: {description}");
                Ok(())
            }
            _ => {
                let display_path = image_path.to_path_buf();

                // Use viuer to display the image with appropriate protocol
                let mut conf = viuer::Config {
                    transparent: true,
                    absolute_offset: false,
                    width: Some(self.max_width.min(80)), // Limit width to 80 columns
                    height: Some(self.max_height.min(24)), // Limit height to 24 rows
                    ..Default::default()
                };

                // Set protocol based on terminal capability
                match self.support {
                    TerminalImageSupport::Kitty => {
                        conf.use_kitty = true;
                        conf.use_iterm = false;
                    }
                    TerminalImageSupport::ITerm2 => {
                        conf.use_kitty = false;
                        conf.use_iterm = true;
                    }
                    _ => {}
                }

                let result = image::open(&display_path)
                    .map_err(anyhow::Error::from)
                    .and_then(|image| viuer::print(&image, &conf).map_err(anyhow::Error::from));
                match result {
                    Ok(_) => {
                        // Print description after the image
                        if !description.is_empty() {
                            println!("📷 {description}");
                        }
                        Ok(())
                    }
                    Err(e) => {
                        // Fallback to text description if image display fails
                        println!("📷 Image: {description} (display failed: {e})");
                        Ok(())
                    }
                }
            }
        }
    }

    /// Render an image from raw bytes
    pub fn render_image_from_bytes(&self, image_data: &[u8], description: &str) -> Result<()> {
        match self.support {
            TerminalImageSupport::None => {
                println!("📷 Image: {description}");
                Ok(())
            }
            _ => {
                let mut conf = viuer::Config {
                    transparent: true,
                    absolute_offset: false,
                    width: Some(self.max_width.min(80)),
                    height: Some(self.max_height.min(24)),
                    ..Default::default()
                };

                // Set protocol based on terminal capability
                match self.support {
                    TerminalImageSupport::Kitty => {
                        conf.use_kitty = true;
                        conf.use_iterm = false;
                    }
                    TerminalImageSupport::ITerm2 => {
                        conf.use_kitty = false;
                        conf.use_iterm = true;
                    }
                    _ => {}
                }

                let result = image::load_from_memory(image_data)
                    .map_err(anyhow::Error::from)
                    .and_then(|image| viuer::print(&image, &conf).map_err(anyhow::Error::from));
                match result {
                    Ok(_) => {
                        if !description.is_empty() {
                            println!("📷 {description}");
                        }
                        Ok(())
                    }
                    Err(e) => {
                        println!("📷 Image: {description} (display failed: {e})");
                        Ok(())
                    }
                }
            }
        }
    }

    /// Get terminal size for image scaling
    fn get_terminal_size() -> (u32, u32) {
        // Try to get terminal size from crossterm
        if let Ok((width, height)) = crossterm::terminal::size() {
            (width as u32, height as u32)
        } else {
            // Fallback to reasonable defaults
            (80, 24)
        }
    }

    /// Print capabilities information for debugging
    pub fn print_capabilities(&self) {
        println!("=== Terminal Image Debug Information ===");
        println!("Detected support: {:?}", self.support);
        println!("Max dimensions: {}x{}", self.max_width, self.max_height);
        println!("Can display images: {}", self.can_display_images());

        // Environment variables
        if let Ok(term) = std::env::var("TERM") {
            println!("TERM: {term}");
        } else {
            println!("TERM: not set");
        }

        if let Ok(term_program) = std::env::var("TERM_PROGRAM") {
            println!("TERM_PROGRAM: {term_program}");
        } else {
            println!("TERM_PROGRAM: not set");
        }

        // Viuer capabilities
        println!(
            "viuer::is_iterm_supported(): {}",
            viuer::is_iterm_supported()
        );

        // Additional debug info
        if let Ok(colorterm) = std::env::var("COLORTERM") {
            println!("COLORTERM: {colorterm}");
        }

        println!("========================================");
    }

    /// Debug method to test image rendering
    pub fn debug_render(&self) {
        println!(
            "DEBUG: Attempting to render test image with support: {:?}",
            self.support
        );
    }
}

impl Default for TerminalImageRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_detection() {
        // This will vary by environment, but should not panic
        let support = TerminalImageRenderer::detect_capabilities();
        println!("Detected support: {support:?}");
    }

    #[test]
    fn test_renderer_creation() {
        let renderer = TerminalImageRenderer::new();
        assert!(renderer.max_width > 0);
        assert!(renderer.max_height > 0);
    }

    #[test]
    fn test_can_display_images() {
        let renderer = TerminalImageRenderer::with_support(TerminalImageSupport::Kitty);
        assert!(renderer.can_display_images());

        let renderer = TerminalImageRenderer::with_support(TerminalImageSupport::None);
        assert!(!renderer.can_display_images());
    }
}
