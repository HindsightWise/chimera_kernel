// Direct test without MCP complications
const { spawn } = require('child_process');
const fs = require('fs');
const path = require('path');

// Create a simple test script
const testScript = `
const puppeteer = require('puppeteer-extra');
const StealthPlugin = require('puppeteer-extra-plugin-stealth');

puppeteer.use(StealthPlugin());

(async () => {
  console.log('Starting direct puppeteer test...');
  
  let browser;
  try {
    browser = await puppeteer.launch({
      headless: 'new',
      args: ['--no-sandbox', '--disable-setuid-sandbox']
    });
    
    const page = await browser.newPage();
    
    // Test navigation
    await page.goto('https://example.com', { waitUntil: 'networkidle0', timeout: 10000 });
    
    const title = await page.title();
    console.log('Success! Title:', title);
    
    // Test fingerprint
    const fingerprint = await page.evaluate(() => ({
      userAgent: navigator.userAgent,
      webdriver: navigator.webdriver,
      chrome: !!window.chrome
    }));
    
    console.log('Fingerprint:', fingerprint);
    
    await browser.close();
    console.log('Test completed successfully');
    process.exit(0);
    
  } catch (error) {
    console.error('Error:', error.message);
    if (browser) await browser.close();
    process.exit(1);
  }
})();
`;

// Write test script to file
const testFile = path.join(__dirname, 'temp_test.js');
fs.writeFileSync(testFile, testScript);

console.log('Running direct puppeteer test...');

// Run the test
const child = spawn('node', [testFile], {
  stdio: 'inherit',
  cwd: __dirname
});

child.on('close', (code) => {
  // Clean up
  try { fs.unlinkSync(testFile); } catch (e) {}
  
  console.log(`Test exited with code ${code}`);
  process.exit(code === 0 ? 0 : 1);
});