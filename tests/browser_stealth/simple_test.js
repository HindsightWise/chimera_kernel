const puppeteer = require('puppeteer');

async function simpleTest() {
  console.log('Starting simple puppeteer test...');
  
  let browser;
  try {
    browser = await puppeteer.launch({
      headless: 'new',
      args: ['--no-sandbox', '--disable-setuid-sandbox']
    });
    
    const page = await browser.newPage();
    await page.goto('https://bot.sannysoft.com', { waitUntil: 'networkidle2', timeout: 10000 });
    
    await page.waitForTimeout(2000);
    
    const title = await page.title();
    console.log('Page title:', title);
    
    // Get some basic info
    const info = await page.evaluate(() => {
      return {
        userAgent: navigator.userAgent,
        webdriver: navigator.webdriver,
        pluginsCount: navigator.plugins.length
      };
    });
    
    console.log('User Agent:', info.userAgent);
    console.log('WebDriver:', info.webdriver);
    console.log('Plugins count:', info.pluginsCount);
    
    await browser.close();
    console.log('Test completed successfully!');
    return true;
    
  } catch (error) {
    console.error('Error in simple test:', error);
    if (browser) await browser.close();
    return false;
  }
}

simpleTest().then(success => {
  process.exit(success ? 0 : 1);
});