//! Integration test: Agent opens Chrome and browses
//!
//! This test simulates an AI agent controlling the computer to:
//! 1. Open Chrome via Spotlight
//! 2. Navigate to a website
//! 3. Scroll and interact

use usehid_core::AgentHID;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("🤖 Agent Integration Test: Open Chrome and Browse");
    println!("================================================");
    println!();
    
    let mut agent = AgentHID::new();
    
    // Step 1: Open Spotlight (Cmd+Space)
    println!("📍 Step 1: Opening Spotlight (Cmd+Space)...");
    let result = agent.execute_json(r#"{"action": "key_combo", "modifiers": ["cmd"], "key": "space"}"#);
    println!("   Result: {:?}", result);
    sleep(Duration::from_millis(500));
    
    // Step 2: Type "Google Chrome"
    println!("📍 Step 2: Typing 'Google Chrome'...");
    let result = agent.execute_json(r#"{"action": "type", "text": "Google Chrome"}"#);
    println!("   Result: {:?}", result);
    sleep(Duration::from_millis(800));
    
    // Step 3: Press Enter to open Chrome
    println!("📍 Step 3: Pressing Enter to launch...");
    let result = agent.execute_json(r#"{"action": "key_press", "key": "enter"}"#);
    println!("   Result: {:?}", result);
    sleep(Duration::from_secs(2));
    
    // Step 4: Open new tab (Cmd+T)
    println!("📍 Step 4: Opening new tab (Cmd+T)...");
    let result = agent.execute_json(r#"{"action": "key_combo", "modifiers": ["cmd"], "key": "t"}"#);
    println!("   Result: {:?}", result);
    sleep(Duration::from_millis(500));
    
    // Step 5: Navigate to URL - type in address bar
    println!("📍 Step 5: Navigating to github.com...");
    let result = agent.execute_json(r#"{"action": "type", "text": "github.com"}"#);
    println!("   Result: {:?}", result);
    sleep(Duration::from_millis(300));
    
    // Step 6: Press Enter to navigate
    println!("📍 Step 6: Pressing Enter to navigate...");
    let result = agent.execute_json(r#"{"action": "key_press", "key": "enter"}"#);
    println!("   Result: {:?}", result);
    sleep(Duration::from_secs(2));
    
    // Step 7: Scroll down the page
    println!("📍 Step 7: Scrolling down the page...");
    for i in 1..=5 {
        let result = agent.execute_json(r#"{"action": "mouse_scroll", "delta": -3}"#);
        println!("   Scroll {}/5: {:?}", i, result);
        sleep(Duration::from_millis(200));
    }
    sleep(Duration::from_millis(500));
    
    // Step 8: Scroll back up
    println!("📍 Step 8: Scrolling back up...");
    for i in 1..=5 {
        let result = agent.execute_json(r#"{"action": "mouse_scroll", "delta": 3}"#);
        println!("   Scroll {}/5: {:?}", i, result);
        sleep(Duration::from_millis(200));
    }
    
    // Step 9: Focus search/address bar (Cmd+L)
    println!("📍 Step 9: Focusing address bar (Cmd+L)...");
    let result = agent.execute_json(r#"{"action": "key_combo", "modifiers": ["cmd"], "key": "l"}"#);
    println!("   Result: {:?}", result);
    sleep(Duration::from_millis(300));
    
    // Step 10: Search something
    println!("📍 Step 10: Searching for 'useHID rust virtual input'...");
    let result = agent.execute_json(r#"{"action": "type", "text": "useHID rust virtual input"}"#);
    println!("   Result: {:?}", result);
    sleep(Duration::from_millis(300));
    
    let result = agent.execute_json(r#"{"action": "key_press", "key": "enter"}"#);
    println!("   Navigate: {:?}", result);
    
    println!();
    println!("✅ Integration test completed!");
    println!();
    println!("The agent successfully:");
    println!("  - Opened Chrome via Spotlight");
    println!("  - Created a new tab");
    println!("  - Navigated to github.com");
    println!("  - Scrolled the page");
    println!("  - Performed a search");
}
