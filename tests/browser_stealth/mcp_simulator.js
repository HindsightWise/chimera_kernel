const puppeteer = require('puppeteer-extra');
const StealthPlugin = require('puppeteer-extra-plugin-stealth');

// Use stealth plugin
puppeteer.use(StealthPlugin());

async function executeStealthBrowser(args) {
  const { url, testMode = false, headless = true } = args;
  
  console.log('=== MCP SIMULATOR TEST ===');
  console.log('Testing with same config as successful direct test...');
  console.log('URL:', url);
  console.log('Test Mode:', testMode);
  console.log('Headless:', headless);
  
  let browser;
  try {
    // Use EXACTLY the same configuration as successful test
    browser = await puppeteer.launch({
      headless: 'new',
      executablePath: '/Users/zerbytheboss/Monad/tests/browser_stealth/chrome-headless-shell/mac_arm-149.0.7803.0/chrome-headless-shell-mac-arm64/chrome-headless-shell',
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

    console.log('Navigating to bot.sannysoft.com...');
    await page.goto('https://bot.sannysoft.com', {
      waitUntil: 'networkidle2',
      timeout: 30000
    });

    // Wait for tests to load using promise-based delay
    await new Promise(resolve => setTimeout(resolve, 5000));

    // Extract test results
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
            const result = cells[1].textContent.trim();
            
            if (testName && result) {
              results[testName] = result;
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
    
    console.log('\n=== RESULTS ===');
    console.log('Detection Rate:', detectionRate + '%');
    console.log('Stealth Effectiveness:', (100 - detectionRate).toFixed(2) + '%');
    console.log('Tests Run:', totalTests);
    console.log('Detection Score:', detectionScore);
    console.log('User Agent:', testResults.userAgent);
    console.log('WebDriver:', testResults.webdriver);
    
    await browser.close();
    
    console.log('\n✅ MCP SIMULATION SUCCESS');
    console.log('Expected MCP integration to work with these results');
    
    return {
      success: true,
      detectionRate: parseFloat(detectionRate),
      detectionScore,
      totalTests,
      results: testResults
    };

  } catch (error) {
    console.error('❌ Error in MCP simulation:', error.message);
    console.error(error.stack);
    if (browser) {
      await browser.close();
    }
    return {
      success: false,
      error: error.message
    };
  }
}

// Test the function
executeStealthBrowser({
  url: 'https://bot.sannysoft.com',
  testMode: true,
  headless: true
}).then(result => {
  console.log('\n=== FINAL VERDICT ===');
  if (result.success) {
    console.log('✅ MCP LOGIC WORKS');
    console.log('The core browser automation logic is functional.');
    console.log('MCP integration failure would be in protocol layer, not browser logic.');
    process.exit(0);
  } else {
    console.log('❌ MCP LOGIC FAILED');
    console.log('Browser automation itself is broken in MCP context.');
    process.exit(1);
  }
}).catch(error => {
  console.error('Unhandled error:', error);
  process.exit(1);
});
