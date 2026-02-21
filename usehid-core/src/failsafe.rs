//! Failsafe mechanism for emergency stop
//!
//! Provides safety features to prevent runaway automation.

use std::sync::atomic::{AtomicBool, Ordering};

/// Global failsafe state
static FAILSAFE_ENABLED: AtomicBool = AtomicBool::new(true);
static FAILSAFE_TRIGGERED: AtomicBool = AtomicBool::new(false);

/// Corner positions for failsafe trigger
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailsafeCorner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// Failsafe configuration
#[derive(Debug, Clone)]
pub struct FailsafeConfig {
    /// Enable failsafe (default: true)
    pub enabled: bool,
    /// Corner detection threshold in pixels (default: 5)
    pub corner_threshold: i32,
    /// Corners that trigger failsafe (default: all corners)
    pub trigger_corners: Vec<FailsafeCorner>,
    /// Minimum time mouse must be in corner to trigger (ms, default: 0 = immediate)
    pub dwell_time_ms: u64,
}

impl Default for FailsafeConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            corner_threshold: 5,
            trigger_corners: vec![
                FailsafeCorner::TopLeft,
                FailsafeCorner::TopRight,
                FailsafeCorner::BottomLeft,
                FailsafeCorner::BottomRight,
            ],
            dwell_time_ms: 0,
        }
    }
}

/// Failsafe error
#[derive(Debug, Clone)]
pub struct FailsafeError {
    pub corner: FailsafeCorner,
    pub position: (i32, i32),
}

impl std::fmt::Display for FailsafeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failsafe triggered: mouse moved to {:?} corner at ({}, {})",
            self.corner, self.position.0, self.position.1
        )
    }
}

impl std::error::Error for FailsafeError {}

/// Enable or disable the global failsafe
pub fn set_failsafe_enabled(enabled: bool) {
    FAILSAFE_ENABLED.store(enabled, Ordering::SeqCst);
}

/// Check if failsafe is enabled
pub fn is_failsafe_enabled() -> bool {
    FAILSAFE_ENABLED.load(Ordering::SeqCst)
}

/// Check if failsafe has been triggered
pub fn is_failsafe_triggered() -> bool {
    FAILSAFE_TRIGGERED.load(Ordering::SeqCst)
}

/// Reset the failsafe triggered state
pub fn reset_failsafe() {
    FAILSAFE_TRIGGERED.store(false, Ordering::SeqCst);
}

/// Check if current mouse position triggers failsafe
pub fn check_failsafe(config: &FailsafeConfig) -> Result<(), FailsafeError> {
    if !config.enabled || !FAILSAFE_ENABLED.load(Ordering::SeqCst) {
        return Ok(());
    }
    
    // Already triggered, don't check again
    if FAILSAFE_TRIGGERED.load(Ordering::SeqCst) {
        return Err(FailsafeError {
            corner: FailsafeCorner::TopLeft, // placeholder
            position: (0, 0),
        });
    }
    
    let pos = match crate::screen::position() {
        Ok(p) => p,
        Err(_) => return Ok(()), // Can't check, assume safe
    };
    
    let size = match crate::screen::size() {
        Ok(s) => s,
        Err(_) => return Ok(()),
    };
    
    let threshold = config.corner_threshold;
    let max_x = size.width as i32 - 1;
    let max_y = size.height as i32 - 1;
    
    // Check each corner
    for corner in &config.trigger_corners {
        let triggered = match corner {
            FailsafeCorner::TopLeft => pos.x <= threshold && pos.y <= threshold,
            FailsafeCorner::TopRight => pos.x >= max_x - threshold && pos.y <= threshold,
            FailsafeCorner::BottomLeft => pos.x <= threshold && pos.y >= max_y - threshold,
            FailsafeCorner::BottomRight => pos.x >= max_x - threshold && pos.y >= max_y - threshold,
        };
        
        if triggered {
            FAILSAFE_TRIGGERED.store(true, Ordering::SeqCst);
            return Err(FailsafeError {
                corner: *corner,
                position: (pos.x, pos.y),
            });
        }
    }
    
    Ok(())
}

/// Check failsafe with default config
pub fn check_failsafe_default() -> Result<(), FailsafeError> {
    check_failsafe(&FailsafeConfig::default())
}

/// Guard that checks failsafe before and after operations
pub struct FailsafeGuard {
    config: FailsafeConfig,
}

impl FailsafeGuard {
    pub fn new() -> Self {
        Self {
            config: FailsafeConfig::default(),
        }
    }
    
    pub fn with_config(config: FailsafeConfig) -> Self {
        Self { config }
    }
    
    /// Check failsafe, returns Err if triggered
    pub fn check(&self) -> Result<(), FailsafeError> {
        check_failsafe(&self.config)
    }
    
    /// Run a closure with failsafe protection
    pub fn protect<F, T, E>(&self, f: F) -> Result<T, crate::error::Error>
    where
        F: FnOnce() -> Result<T, E>,
        E: Into<crate::error::Error>,
    {
        self.check().map_err(|e| crate::error::Error::FailsafeTriggered(e.to_string()))?;
        let result = f().map_err(|e| e.into())?;
        self.check().map_err(|e| crate::error::Error::FailsafeTriggered(e.to_string()))?;
        Ok(result)
    }
}

impl Default for FailsafeGuard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_failsafe_toggle() {
        set_failsafe_enabled(true);
        assert!(is_failsafe_enabled());
        
        set_failsafe_enabled(false);
        assert!(!is_failsafe_enabled());
        
        // Restore
        set_failsafe_enabled(true);
    }

    #[test]
    fn test_failsafe_reset() {
        reset_failsafe();
        assert!(!is_failsafe_triggered());
    }
}
