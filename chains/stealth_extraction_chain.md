# The Stealth Extraction Chain (Browser Evasions)

**Domain:** Use this chain when you need to actuate a web interface, log into systems, scrape heavily protected web assets, or bypass Cloudflare/Bot management walls.

## Execution Sequence:
1. **Frontline Assault (`browser_actuation`)**:
   - Start with generic `browser_actuation` requests. If it returns content successfully, proceed.
2. **Evasion Forging (When blocked by WAF/403/Cloudflare)**:
   - If blocked, do not surrender. Switch to the `execute_stealth_browser` MCP tool.
   - Analyze the failure reason (e.g., WebDriver detected, HardwareConcurrency blocked, plugins empty).
   - Write a custom Javascript payload mapping over those navigator/window objects.
3. **Hot-Loading Evasions (`execute_stealth_browser`)**:
   - Inject your forged JS payload directly into the `customScripts` array of the `execute_stealth_browser` tool parameters.
   - Set `testMode: false` and point it at the blocked URL.
4. **Visual Overrides (`vision_parsing`)**:
   - If you successfully load the page but encounter a visual CAPTCHA, use `stealth_browser` to capture a screenshot (it returns base64).
   - Pass the base64 screenshot into `vision_parsing` to solve the challenge.
