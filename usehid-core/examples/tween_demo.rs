//! Example: Smooth mouse movement with tweening
//!
//! Run with: cargo run --release -p usehid --example tween_demo

use usehid::{size, position, move_to, AgentHID, Tween, TweenAnimation};

fn main() {
    println!("=== useHID Tween Demo ===\n");

    // Get screen size and center
    let screen = size().expect("Failed to get screen size");
    let center_x = screen.width as i32 / 2;
    let center_y = screen.height as i32 / 2;
    
    println!("Screen: {}x{}", screen.width, screen.height);
    println!("Center: ({}, {})\n", center_x, center_y);

    // Demo 1: Direct tween API
    println!("--- Demo 1: Direct Tween API ---");
    
    let pos = position().expect("Failed to get position");
    println!("Start position: ({}, {})", pos.x, pos.y);
    
    // Animate to center using EaseInOutQuad
    let anim = TweenAnimation::new(
        pos.x as f64, pos.y as f64,
        center_x as f64, center_y as f64,
        1000,
        Tween::EaseInOutQuad,
    );
    
    let positions = anim.generate_positions(60);
    println!("Animating {} frames over 1 second...", positions.len());
    
    let frame_delay = std::time::Duration::from_millis(1000 / 60);
    for (x, y) in positions {
        move_to(x, y).ok();
        std::thread::sleep(frame_delay);
    }
    
    let pos = position().expect("Failed to get position");
    println!("End position: ({}, {})\n", pos.x, pos.y);

    // Demo 2: Agent API with duration/tween
    println!("--- Demo 2: Agent API with Duration ---");
    
    let mut agent = AgentHID::new();
    
    // Move to corner
    agent.execute_json(r#"{"action": "mouse_move_to", "x": 100, "y": 100}"#);
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    // Smooth move to center with easing
    println!("Moving to center with EaseOutCubic over 2 seconds...");
    let json = format!(
        r#"{{"action": "mouse_move_to", "x": {}, "y": {}, "duration": 2000, "tween": "ease_out_cubic"}}"#,
        center_x, center_y
    );
    let result = agent.execute_json(&json);
    println!("Result: {:?}\n", result);

    // Demo 3: Different tween functions
    println!("--- Demo 3: Tween Comparisons ---");
    
    let tweens = [
        ("Linear", Tween::Linear),
        ("EaseInQuad", Tween::EaseInQuad),
        ("EaseOutQuad", Tween::EaseOutQuad),
        ("EaseInOutQuad", Tween::EaseInOutQuad),
        ("EaseOutBounce", Tween::EaseOutBounce),
        ("EaseOutElastic", Tween::EaseOutElastic),
    ];
    
    for (name, tween) in tweens {
        println!("\n{}: t=0.25 -> {:.3}, t=0.5 -> {:.3}, t=0.75 -> {:.3}",
            name,
            tween.apply(0.25),
            tween.apply(0.5),
            tween.apply(0.75)
        );
    }

    // Demo 4: Type with interval
    println!("\n\n--- Demo 4: Typing with Interval ---");
    println!("Typing 'Hello' with 250ms interval...");
    let result = agent.execute_json(r#"{"action": "type", "text": "Hello", "interval": 250}"#);
    println!("Result: {:?}", result);

    println!("\nDone!");
}
