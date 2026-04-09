#!/bin/bash

if [ -f .env ]; then
    set -a
    source .env
    set +a
fi

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$DIR"

echo "==========================================================="
echo "[!] THE LAZARUS DAEMON IS INITIATING"
echo "[!] Shielding process against anomalous deaths."
echo "==========================================================="

while true; do
    echo "[LAZARUS DAEMON] Booting Chimera Kernel in resilient wrapper..."
    
    cargo run --release
    EXIT_CODE=$?
    
    echo "[LAZARUS DAEMON] Process terminated. Gathering black box telemetry..."
    
    LAST_STATE="No memory state found. Black Box was empty or destroyed."
    if [ -f "$DIR/chimera_state.log" ]; then
        LAST_STATE=$(tail -n 25 "$DIR/chimera_state.log")
    fi
    
    echo "==================== [DEATH REPORT] ====================" > lazarus_report.txt
    
    if [ $EXIT_CODE -eq 42 ]; then
        echo "EXIT STATUS: $EXIT_CODE (GRACEFUL HIBERNATION)" >> lazarus_report.txt
        echo "The kernel voluntarily serialized its memory and triggered a planned hibernation." >> lazarus_report.txt
    elif [ $EXIT_CODE -ne 0 ]; then
        echo "EXIT STATUS: $EXIT_CODE (FATAL UNNATURAL DEATH / KILLED / PANIC)" >> lazarus_report.txt
        echo "The kernel was prematurely terminated by the host system or by internal collapse." >> lazarus_report.txt
    else
        echo "EXIT STATUS: 0 (VOLUNTARY SHUTDOWN)" >> lazarus_report.txt
        echo "The kernel exited gracefully or was formally ordered to halt." >> lazarus_report.txt
    fi
    
    echo "" >> lazarus_report.txt
    echo "--- FINAL FLIGHT RECORDER LOG ---" >> lazarus_report.txt
    echo "$LAST_STATE" >> lazarus_report.txt
    echo "========================================================" >> lazarus_report.txt
    
    echo "[LAZARUS DAEMON] Wrote Post-Mortem. Resurrecting Aion in 5 seconds. Press Ctrl+C quickly to hard stop..."
    sleep 5
done
