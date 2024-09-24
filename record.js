// const puppeteer = require('puppeteer');
const puppeteer = require('puppeteer-extra');
const StealthPlugin = require('puppeteer-extra-plugin-stealth');
puppeteer.use(StealthPlugin());

var target = process.argv[2]
var viewport = { width: 1920, height: 6866 };

console.log(target);

(async () => {
    const page = await browser.newPage();
    page.on('console', (msg) => console.log(msg.text()));
});

(async function () {
    const browser = await puppeteer.launch({
        headless: false,
        args: [`--window-size=${viewport.width},${viewport.height}`,
            `--disable-features=site-per-process`,
            `--disable-fre`,
            `--no-default-browser-check`,
            `--no-first-run`,
            `--ignore-certificate-errors`,
            `--no-sandbox`]
    });
    
    const page = await browser.newPage();
    await page.setViewport(viewport);
    // await page.goto('https://en.wikipedia.org/wiki/%22Hello,_World!%22_program');
    await page.goto(target, { waitUntil: 'networkidle2' });
    await page.screenshot({ path: './screenshot.png' });

    console.log(await page.evaluate(() => document.body.scrollHeight));
    // browser.close();
})();