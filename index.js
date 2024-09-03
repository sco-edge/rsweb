const puppeteer = require('puppeteer');

var viewport = { width: 1920, height: 1080 };

(async function() {
    const browser = await puppeteer.launch({ headless: false,
                                             args: [`--window-size=${viewport.width},${viewport.height}`] });
    const page = await browser.newPage();
    await page.setViewport(viewport);
    await page.goto('https://en.wikipedia.org/wiki/%22Hello,_World!%22_program');
    await page.screenshot({ path: './screenshot.png'});
    // browser.close();
})();