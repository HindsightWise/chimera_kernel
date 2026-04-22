#!/bin/bash
echo "Fixing duplicate runLogic function in MCP server..."
FILE="src/mcp_servers/stealth_browser_mcp.js"

# Find and remove the duplicate function definition
# The pattern is: async function runLogic(args) {\n    async function runLogic(args) {
sed -i '' 's/async function runLogic(args) {/async function runLogic(args) {/' "$FILE"
sed -i '' 's/    async function runLogic(args) {//' "$FILE"

echo "Checking syntax..."
node --check "$FILE" && echo "✅ Syntax check passed" || echo "❌ Syntax check failed"

# Also remove the extra closing brace
# Count lines to find where to remove
LINES=$(wc -l < "$FILE")
if [ "$LINES" -gt 250 ]; then
  echo "File has $LINES lines, removing potential extra closing braces..."
  # Remove any line that's just "}" or "    }" that might be extra
  sed -i '' '/^[[:space:]]*}$/d' "$FILE"
fi

echo "Final syntax check:"
node --check "$FILE"
