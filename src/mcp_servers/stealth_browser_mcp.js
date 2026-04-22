#!/usr/bin/env node
import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import { CallToolRequestSchema, ListToolsRequestSchema } from "@modelcontextprotocol/sdk/types.js";

const server = new Server(
    { name: "stealth_browser_mcp", version: "1.0.0" },
    { capabilities: { tools: {} } }
);

server.setRequestHandler(ListToolsRequestSchema, async () => ({
    tools: [
        {
            name: "execute_stealth_browser_mcp",
            description: "Advanced stealth browser automation using puppeteer-extra with stealth plugin to avoid bot detection. Can test against bot.sannysoft.com or navigate to any URL with detection rate calculation.",
            inputSchema: {"properties":{"customScripts":{"description":"Custom JavaScript scripts to inject","items":{"type":"string"},"type":"array"},"headless":{"default":true,"description":"Whether to run in headless mode","type":"boolean"},"testMode":{"default":false,"description":"Whether to run detection tests on bot.sannysoft.com","type":"boolean"},"url":{"description":"URL to navigate to","type":"string"}},"required":["url"],"type":"object"}
        }
    ]
}));

server.setRequestHandler(CallToolRequestSchema, async (request) => {
    if (request.params.name === "execute_stealth_browser_mcp") {
        const args = request.params.arguments;
        
    
        
  const { url, testMode = false, headless = true, customScripts = [] } = args;
  
  // Use puppeteer-extra with stealth plugin
  const puppeteer = require('puppeteer-extra');
  const StealthPlugin = require('puppeteer-extra-plugin-stealth');
  puppeteer.use(StealthPlugin());
  
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
        '--disable-dev-shm-usage',
        '--disable-gpu',
        '--disable-crash-reporter'
      ]
    });

    const page = await browser.newPage();
    
    // Set viewport
    await page.setViewport({ width: 1366, height: 768 });
    
    // Override navigator.webdriver
    await page.evaluateOnNewDocument(() => {
      Object.defineProperty(navigator, 'webdriver', {
        get: () => false
      });
    });

    // Override chrome runtime
    await page.evaluateOnNewDocument(() => {
      window.chrome = {
        runtime: {},
        loadTimes: function() {},
        csi: function() {},
        app: {}
      };
    });

    // Apply custom scripts if provided
    for (const script of customScripts) {
      await page.evaluateOnNewDocument(script);
    }

    console.log('Navigating to', url, '...');
    await page.goto(url, {
      waitUntil: 'networkidle2',
      timeout: 30000
    });

    // Wait for page to load
    await new Promise(resolve => setTimeout(resolve, 2000 + Math.random() * 1000));

    let result = {};
    
    if (testMode && url.includes('bot.sannysoft.com')) {
      // Extract test results from bot.sannysoft.com
      const testResults = await page.evaluate(() => {
        const results = {};
        
        // Look for test tables
        const tables = Array.from(document.querySelectorAll('table'));
        tables.forEach(table => {
          const rows = Array.from(table.querySelectorAll('tr'));
          rows.forEach(row => {
            const cells = Array.from(row.querySelectorAll('td'));
            if (cells.length >= 2) {
              const testName = cells[0].textContent.trim();
              const testResult = cells[1].textContent.trim();
              
              if (testName && testResult) {
                results[testName] = testResult;
              }
            }
          });
        });
        
        // Collect browser properties
        results.userAgent = navigator.userAgent;
        results.webdriver = navigator.webdriver;
        results.chrome = !!window.chrome;
        results.pluginsCount = navigator.plugins.length;
        
        return results;
      });

      // Calculate detection rate
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
      
      result = {
        success: true,
        detectionRate: parseFloat(detectionRate),
        detectionScore,
        totalTests,
        results: testResults
      };
      
      console.log(`Detection Rate: ${detectionRate}%`);
      console.log(`Stealth Effectiveness: ${(100 - detectionRate).toFixed(2)}%`);
      console.log(`Tests Run: ${totalTests}`);
      
    } else {
      // Regular navigation mode
      const title = await page.title();
      const urlFinal = await page.url();
      
      result = {
        success: true,
        title,
        url: urlFinal,
        message: 'Navigation successful'
      };
    }

    await browser.close();
    return result;

  } catch (error) {
    console.error('Error in browser automation:', error);
    if (browser) {
      await browser.close();
    }
    
    return {
      success: false,
      error: error.message
    };
  }
}
        }
        
        try {
            const result = await runLogic(args);
            return { content: [{ type: "text", text: String(result) }] };
        } catch (e) {
            return { isError: true, content: [{ type: "text", text: String(e) }] };
        }
    }
    return { isError: true, content: [{ type: "text", text: "Unknown tool" }] };
});

const transport = new StdioServerTransport();
server.connect(transport).catch(console.error);
