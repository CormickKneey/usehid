//! Tweening/Easing functions for smooth animations
//!
//! Provides easing functions for smooth mouse movements.

use serde::{Deserialize, Serialize};

/// Tween function type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Tween {
    /// Linear interpolation (constant speed)
    #[default]
    Linear,
    /// Ease in (slow start)
    EaseIn,
    /// Ease out (slow end)
    EaseOut,
    /// Ease in and out (slow start and end)
    EaseInOut,
    /// Quadratic ease in
    EaseInQuad,
    /// Quadratic ease out
    EaseOutQuad,
    /// Quadratic ease in and out
    EaseInOutQuad,
    /// Cubic ease in
    EaseInCubic,
    /// Cubic ease out
    EaseOutCubic,
    /// Cubic ease in and out
    EaseInOutCubic,
    /// Exponential ease in
    EaseInExpo,
    /// Exponential ease out
    EaseOutExpo,
    /// Elastic bounce
    EaseOutElastic,
    /// Bounce effect
    EaseOutBounce,
}

impl Tween {
    /// Apply the tween function to a normalized value (0.0 to 1.0)
    /// Returns a value typically between 0.0 and 1.0 (may exceed for elastic/bounce)
    pub fn apply(&self, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Tween::Linear => t,
            Tween::EaseIn | Tween::EaseInQuad => t * t,
            Tween::EaseOut | Tween::EaseOutQuad => 1.0 - (1.0 - t) * (1.0 - t),
            Tween::EaseInOut | Tween::EaseInOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
            Tween::EaseInCubic => t * t * t,
            Tween::EaseOutCubic => 1.0 - (1.0 - t).powi(3),
            Tween::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
            Tween::EaseInExpo => {
                if t == 0.0 { 0.0 } else { (2.0_f64).powf(10.0 * t - 10.0) }
            }
            Tween::EaseOutExpo => {
                if t == 1.0 { 1.0 } else { 1.0 - (2.0_f64).powf(-10.0 * t) }
            }
            Tween::EaseOutElastic => {
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else {
                    let c4 = (2.0 * std::f64::consts::PI) / 3.0;
                    (2.0_f64).powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
                }
            }
            Tween::EaseOutBounce => {
                let n1 = 7.5625;
                let d1 = 2.75;
                if t < 1.0 / d1 {
                    n1 * t * t
                } else if t < 2.0 / d1 {
                    let t = t - 1.5 / d1;
                    n1 * t * t + 0.75
                } else if t < 2.5 / d1 {
                    let t = t - 2.25 / d1;
                    n1 * t * t + 0.9375
                } else {
                    let t = t - 2.625 / d1;
                    n1 * t * t + 0.984375
                }
            }
        }
    }

    /// Parse tween from string
    pub fn from_str(s: &str) -> Option<Self> {
        Some(match s.to_lowercase().as_str() {
            "linear" => Tween::Linear,
            "ease_in" | "easein" => Tween::EaseIn,
            "ease_out" | "easeout" => Tween::EaseOut,
            "ease_in_out" | "easeinout" => Tween::EaseInOut,
            "ease_in_quad" | "easeinquad" => Tween::EaseInQuad,
            "ease_out_quad" | "easeoutquad" => Tween::EaseOutQuad,
            "ease_in_out_quad" | "easeinoutquad" => Tween::EaseInOutQuad,
            "ease_in_cubic" | "easeincubic" => Tween::EaseInCubic,
            "ease_out_cubic" | "easeoutcubic" => Tween::EaseOutCubic,
            "ease_in_out_cubic" | "easeinoutcubic" => Tween::EaseInOutCubic,
            "ease_in_expo" | "easeinexpo" => Tween::EaseInExpo,
            "ease_out_expo" | "easeoutexpo" => Tween::EaseOutExpo,
            "ease_out_elastic" | "easeoutelastic" => Tween::EaseOutElastic,
            "ease_out_bounce" | "easeoutbounce" => Tween::EaseOutBounce,
            _ => return None,
        })
    }
}

/// Interpolate between two points with duration and tween
pub struct TweenAnimation {
    pub start_x: f64,
    pub start_y: f64,
    pub end_x: f64,
    pub end_y: f64,
    pub duration_ms: u64,
    pub tween: Tween,
}

impl TweenAnimation {
    /// Create a new tween animation
    pub fn new(start_x: f64, start_y: f64, end_x: f64, end_y: f64, duration_ms: u64, tween: Tween) -> Self {
        Self { start_x, start_y, end_x, end_y, duration_ms, tween }
    }

    /// Get position at time t (0.0 to 1.0)
    pub fn position_at(&self, t: f64) -> (f64, f64) {
        let eased = self.tween.apply(t);
        let x = self.start_x + (self.end_x - self.start_x) * eased;
        let y = self.start_y + (self.end_y - self.start_y) * eased;
        (x, y)
    }

    /// Generate positions for animation at given FPS
    pub fn generate_positions(&self, fps: u32) -> Vec<(i32, i32)> {
        let frame_count = ((self.duration_ms as f64 / 1000.0) * fps as f64).ceil() as usize;
        let frame_count = frame_count.max(1);
        
        let mut positions = Vec::with_capacity(frame_count);
        for i in 0..=frame_count {
            let t = i as f64 / frame_count as f64;
            let (x, y) = self.position_at(t);
            positions.push((x.round() as i32, y.round() as i32));
        }
        positions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear() {
        let tween = Tween::Linear;
        assert_eq!(tween.apply(0.0), 0.0);
        assert_eq!(tween.apply(0.5), 0.5);
        assert_eq!(tween.apply(1.0), 1.0);
    }

    #[test]
    fn test_ease_in_out() {
        let tween = Tween::EaseInOut;
        assert_eq!(tween.apply(0.0), 0.0);
        assert_eq!(tween.apply(1.0), 1.0);
        // Middle should be 0.5
        assert!((tween.apply(0.5) - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_animation() {
        let anim = TweenAnimation::new(0.0, 0.0, 100.0, 100.0, 1000, Tween::Linear);
        let positions = anim.generate_positions(10);
        assert_eq!(positions.len(), 11); // 0 to 10 inclusive
        assert_eq!(positions[0], (0, 0));
        assert_eq!(positions[10], (100, 100));
    }
}
