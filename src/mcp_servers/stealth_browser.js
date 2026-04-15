#!/usr/bin/env node
import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import { CallToolRequestSchema, ListToolsRequestSchema } from "@modelcontextprotocol/sdk/types.js";

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
                    url: {
                        type: "string",
                        description: "URL to navigate to"
                    },
                    testMode: {
                        type: "boolean",
                        description: "Whether to run bot.sannysoft.com detection tests",
                        default: false
                    },
                    headless: {
                        type: "boolean",
                        description: "Whether to run in headless mode",
                        default: true
                    }
                },
                required: ["url"]
            }
        }
    ]
}));

server.setRequestHandler(CallToolRequestSchema, async (request) => {
    if (request.params.name === "execute_stealth_browser") {
        const args = request.params.arguments;
        
        async function runLogic(args) {
            // Import puppeteer-extra dynamically since we're in ES module
            const { default: puppeteer } = await import('puppeteer-extra');
            const StealthPlugin = await import('puppeteer-extra-plugin-stealth');
            
            // Use stealth plugin
            puppeteer.use(StealthPlugin.default());
            
            const { url, testMode = false, headless = true } = args;
            
            let browser;
            try {
                // Launch browser with stealth configuration
                browser = await puppeteer.launch({
                    headless: headless ? 'new' : false,
                    executablePath: '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',
                    args: [
                        '--no-sandbox',
                        '--disable-setuid-sandbox',
                        '--disable-web-security',
                        '--disable-features=IsolateOrigins,site-per-process',
                        '--disable-blink-features=AutomationControlled',
                        '--disable-features=AudioServiceOutOfProcess',
                        '--disable-dev-shm-usage',
                        '--disable-gpu',
                        '--window-size=1366,768'
                    ]
                });

                const page = await browser.newPage();
                
                // Set viewport
                await page.setViewport({ width: 1366, height: 768 });
                
                // Navigate to URL
                await page.goto(url, {
                    waitUntil: 'networkidle2',
                    timeout: 30000
                });

                // Wait a bit for page to fully load
                await page.waitForTimeout(2000);

                if (testMode) {
                    // For bot.sannysoft.com testing
                    const testResults = await page.evaluate(() => {
                        const results = {};
                        
                        // Get all test results
                        const tables = document.querySelectorAll('table');
                        tables.forEach((table) => {
                            const rows = table.querySelectorAll('tr');
                            
                            rows.forEach(row => {
                                const cells = row.querySelectorAll('td, th');
                                if (cells.length >= 2) {
                                    const testName = cells[0].textContent.trim();
                                    const testResult = cells[1].textContent.trim();
                                    
                                    if (testName && testResult && !testName.includes('Test Name')) {
                                        results[testName] = testResult;
                                    }
                                }
                            });
                        });

                        // Browser fingerprint
                        const fingerprint = {
                            userAgent: navigator.userAgent,
                            webdriver: navigator.webdriver,
                            chrome: !!window.chrome,
                            plugins: {
                                length: navigator.plugins.length,
                                names: Array.from(navigator.plugins).map(p => p.name).join(', ')
                            },
                            languages: navigator.languages.join(', '),
                            platform: navigator.platform,
                            hardwareConcurrency: navigator.hardwareConcurrency,
                            deviceMemory: navigator.deviceMemory,
                            screen: {
                                width: screen.width,
                                height: screen.height,
                                colorDepth: screen.colorDepth
                            },
                            timezone: Intl.DateTimeFormat().resolvedOptions().timeZone
                        };

                        return {
                            testResults,
                            fingerprint,
                            totalTests: Object.keys(results).length,
                            detectedTests: Object.values(results).filter(r => 
                                r.includes('failed') || r.includes('present') || r.includes('detected') || r === 'prompt'
                            ).length
                        };
                    });

                    // Take screenshot
                    const screenshot = await page.screenshot({ encoding: 'base64' });
                    
                    await browser.close();
                    
                    return {
                        success: true,
                        testResults,
                        screenshot: `data:image/png;base64,${screenshot}`,
                        detectionRate: testResults.detectedTests / testResults.totalTests * 100,
                        stealthEffectiveness: 100 - (testResults.detectedTests / testResults.totalTests * 100)
                    };
                } else {
                    // Regular navigation - get page content
                    const content = await page.content();
                    const title = await page.title();
                    
                    await browser.close();
                    
                    return {
                        success: true,
                        title,
                        contentLength: content.length,
                        url: page.url()
                    };
                }
                
            } catch (error) {
                if (browser) {
                    await browser.close();
                }
                throw new Error(`Stealth browser error: ${error.message}`);
            }
        }
        
        try {
            const result = await runLogic(args);
            return { content: [{ type: "text", text: JSON.stringify(result, null, 2) }] };
        } catch (e) {
            return { isError: true, content: [{ type: "text", text: String(e) }] };
        }
    }
    return { isError: true, content: [{ type: "text", text: "Unknown tool" }] };
});

const transport = new StdioServerTransport();
server.connect(transport).catch(console.error);