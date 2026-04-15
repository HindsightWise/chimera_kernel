#!/bin/bash

# The session name for the virtual terminal
SESSION_NAME="chimera"

# Check if a session with this name is already running
if tmux has-session -t $SESSION_NAME 2>/dev/null; then
    echo "⚠️  The Swarm is already running in the background!"
    echo "👉 To view it, run: tmux attach -t $SESSION_NAME"
    exit 0
fi

echo "🚀 Booting Chimera Kernel inside a detached virtual terminal..."

# Create the session detached (-d), name it (-s), and execute the cargo command
tmux new-session -d -s $SESSION_NAME 'cargo run --release'

echo "✅ Swarm is now humming in the background!"
echo ""
echo "You can completely close this terminal window right now."
echo ""
echo "Commands to remember:"
echo "👁️  To look at the logs:  tmux attach -t $SESSION_NAME"
echo "🚪 To leave the logs:   Press 'Control + B', let go, then press 'D'"
echo "💀 To kill the swarm:   tmux kill-session -t $SESSION_NAME"
