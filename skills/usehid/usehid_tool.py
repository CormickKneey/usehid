#!/usr/bin/env python3
"""
useHID Tool for OpenClaw

Usage:
    python usehid_tool.py '{"action": "mouse_click", "button": "left"}'
    python usehid_tool.py '{"action": "type", "text": "Hello"}'
"""

import sys
import json

def main():
    if len(sys.argv) < 2:
        print(json.dumps({"error": "No action provided", "usage": "python usehid_tool.py '{\"action\": \"...\"}'"}, indent=2))
        sys.exit(1)
    
    try:
        action = json.loads(sys.argv[1])
    except json.JSONDecodeError as e:
        print(json.dumps({"error": f"Invalid JSON: {e}"}))
        sys.exit(1)
    
    try:
        from usehid import AgentHID
        agent = AgentHID()
        result = agent.execute(action)
        print(json.dumps({"success": True, "result": result}))
    except ImportError:
        # Fallback: try to use the Rust binary directly via subprocess
        import subprocess
        import os
        
        # Try to find usehid-core example
        usehid_dir = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
        agent_bin = os.path.join(usehid_dir, "target", "release", "examples", "agent")
        
        if os.path.exists(agent_bin):
            result = subprocess.run(
                [agent_bin],
                input=json.dumps(action),
                capture_output=True,
                text=True
            )
            print(json.dumps({"success": result.returncode == 0, "output": result.stdout, "error": result.stderr}))
        else:
            print(json.dumps({
                "error": "usehid not installed",
                "install": "cd usehid-python && pip install maturin && maturin develop --release"
            }))
            sys.exit(1)
    except Exception as e:
        print(json.dumps({"error": str(e)}))
        sys.exit(1)

if __name__ == "__main__":
    main()
