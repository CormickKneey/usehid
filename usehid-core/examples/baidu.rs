//! Open Chrome and navigate to baidu.com

use usehid_core::AgentHID;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("🤖 Opening Chrome and navigating to baidu.com");
    println!("==============================================");
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
    println!("📍 Step 3: Pressing Enter to launch Chrome...");
    let result = agent.execute_json(r#"{"action": "key_press", "key": "enter"}"#);
    println!("   Result: {:?}", result);
    sleep(Duration::from_secs(2));
    
    // Step 4: Focus address bar (Cmd+L)
    println!("📍 Step 4: Focusing address bar (Cmd+L)...");
    let result = agent.execute_json(r#"{"action": "key_combo", "modifiers": ["cmd"], "key": "l"}"#);
    println!("   Result: {:?}", result);
    sleep(Duration::from_millis(300));
    
    // Step 5: Type baidu.com
    println!("📍 Step 5: Typing 'baidu.com'...");
    let result = agent.execute_json(r#"{"action": "type", "text": "baidu.com"}"#);
    println!("   Result: {:?}", result);
    sleep(Duration::from_millis(300));
    
    // Step 6: Press Enter to navigate
    println!("📍 Step 6: Pressing Enter to navigate...");
    let result = agent.execute_json(r#"{"action": "key_press", "key": "enter"}"#);
    println!("   Result: {:?}", result);
    
    println!();
    println!("✅ Done! Chrome should now show baidu.com");
}
