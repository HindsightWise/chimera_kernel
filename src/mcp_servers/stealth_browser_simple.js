#!/usr/bin/env node
const { spawn } = require('child_process');
const path = require('path');

// Simple MCP server that calls the working test script
const { Server } = require('@modelcontextprotocol/sdk/server/index.js');
const { StdioServerTransport } = require('@modelcontextprotocol/sdk/server/stdio.js');
const { CallToolRequestSchema, ListToolsRequestSchema } = require('@modelcontextprotocol/sdk/types.js');

const server = new Server(
    { name: "stealth_browser", version: "1.0.0" },
    { capabilities: { tools: {} } }
);

server.setRequestHandler(ListToolsRequestSchema, async () => ({
    tools: [
        {
            name: "execute_stealth_browser",
            description: "Advanced stealth browser automation using puppeteer-extra with stealth plugin to avoid bot detection. Can test against bot.sannysoft.com or navigate to any URL.",
            inputSchema: {
                type: "object",
                properties: {
                    url: { type: "string", description: "URL to navigate to" },
                    testMode: { type: "boolean", description: "Whether to run detection tests on bot.sannysoft.com", default: false },
                    headless: { type: "boolean", description: "Whether to run in headless mode", default: true },
                    customScripts: { type: "array", items: { type: "string" }, description: "Custom JavaScript scripts to inject" }
                },
                required: ["url"]
            }
        }
    ]
}));

server.setRequestHandler(CallToolRequestSchema, async (request) => {
    if (request.params.name === "execute_stealth_browser") {
        const args = request.params.arguments;
        
        return new Promise((resolve) => {
            try {
                const { url, testMode = false } = args;
                
                // Create a temporary test script
                const testScript = `
const puppeteer = require('puppeteer-extra');
const StealthPlugin = require('puppeteer-extra-plugin-stealth');
puppeteer.use(StealthPlugin());

(async () => {
    let browser;
    try {
        browser = await puppeteer.launch({
            headless: 'new',
            executablePath: '${path.join(__dirname, '../../tests/browser_stealth/chrome-headless-shell/mac_arm-149.0.7803.0/chrome-headless-shell-mac-arm64/chrome-headless-shell')}',
            args: [
                '--no-sandbox',
                '--disable-setuid-sandbox',
                '--disable-web-security',
                '--disable-features=IsolateOrigins,site-per-process',
                '--disable-blink-features=AutomationControlled',
                '--disable-dev-shm-usage',
                '--disable-gpu',
                '--disable-crash-reporter'
            ]
        });

        const page = await browser.newPage();
        await page.setViewport({ width: 1366, height: 768 });
        
        await page.evaluateOnNewDocument(() => {
            Object.defineProperty(navigator, 'webdriver', { get: () => false });
        });

        await page.evaluateOnNewDocument(() => {
            window.chrome = { runtime: {}, loadTimes: function() {}, csi: function() {}, app: {} };
        });

        await page.goto('${url}', { waitUntil: 'networkidle2', timeout: 30000 });
        await new Promise(resolve => setTimeout(resolve, 5000));

        const testResults = await page.evaluate(() => {
            const results = {};
            const tables = Array.from(document.querySelectorAll('table'));
            tables.forEach(table => {
                const rows = Array.from(table.querySelectorAll('tr'));
                rows.forEach(row => {
                    const cells = Array.from(row.querySelectorAll('td'));
                    if (cells.length >= 2) {
                        const testName = cells[0].textContent.trim();
                        const result = cells[1].textContent.trim();
                        if (testName && result) results[testName] = result;
                    }
                });
            });
            
            results.userAgent = navigator.userAgent;
            results.webdriver = navigator.webdriver;
            results.chrome = !!window.chrome;
            results.pluginsCount = navigator.plugins.length;
            return results;
        });

        let detectionScore = 0;
        let totalTests = 0;
        Object.entries(testResults).forEach(([key, value]) => {
            if (key !== 'userAgent' && key !== 'webdriver' && key !== 'chrome' && key !== 'pluginsCount') {
                totalTests++;
                if (value.toLowerCase().includes('failed') || 
                    value.toLowerCase().includes('detected') ||
                    value.toLowerCase().includes('fake')) {
                    detectionScore++;
                }
            }
        });

        const detectionRate = totalTests > 0 ? (detectionScore / totalTests * 100).toFixed(2) : 0;
        
        await browser.close();
        
        console.log(JSON.stringify({
            success: true,
            detectionRate: parseFloat(detectionRate),
            detectionScore,
            totalTests,
            results: testResults
        }));
        
    } catch (error) {
        console.error(JSON.stringify({
            success: false,
            error: error.message
        }));
        if (browser) await browser.close();
    }
})();
`;
                
                // Write and execute the test
                const testFile = path.join(__dirname, 'stealth_test_temp.js');
                require('fs').writeFileSync(testFile, testScript);
                
                const child = spawn('node', [testFile], {
                    cwd: path.join(__dirname, '../../tests/browser_stealth'),
                    stdio: ['pipe', 'pipe', 'pipe'],
                    timeout: 60000
                });
                
                let stdout = '';
                let stderr = '';
                
                child.stdout.on('data', (data) => {
                    stdout += data.toString();
                });
                
                child.stderr.on('data', (data) => {
                    stderr += data.toString();
                });
                
                child.on('close', (code) => {
                    try {
                        const cleanStdout = stdout.trim();
                        let result;
                        
                        // Try to parse JSON output
                        try {
                            const jsonMatch = cleanStdout.match(/\{[\s\S]*\}/);
                            if (jsonMatch) {
                                result = JSON.parse(jsonMatch[0]);
                            } else {
                                throw new Error('No JSON output found');
                            }
                        } catch (e) {
                            result = {
                                success: false,
                                error: `Failed to parse output: ${e.message}`,
                                rawOutput: cleanStdout,
                                stderr: stderr
                            };
                        }
                        
                        resolve({
                            content: [{
                                type: "text",
                                text: JSON.stringify(result, null, 2)
                            }]
                        });
                        
                    } catch (error) {
                        resolve({
                            content: [{
                                type: "text",
                                text: JSON.stringify({
                                    success: false,
                                    error: error.message,
                                    stdout: stdout,
                                    stderr: stderr
                                }, null, 2)
                            }]
                        });
                    }
                    
                    // Clean up
                    try { require('fs').unlinkSync(testFile); } catch (e) {}
                });
                
                child.on('error', (error) => {
                    resolve({
                        content: [{
                            type: "text",
                            text: JSON.stringify({
                                success: false,
                                error: error.message
                            }, null, 2)
                        }]
                    });
                });
                
            } catch (error) {
                resolve({
                    content: [{
                        type: "text",
                        text: JSON.stringify({
                            success: false,
                            error: error.message
                        }, null, 2)
                    }]
                });
            }
        });
    }
});

const transport = new StdioServerTransport();
server.connect(transport).catch(console.error);
