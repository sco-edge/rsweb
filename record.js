const puppeteer = require('puppeteer');

var target = process.argv[2]
var viewport = { width: 1920, height: 1080 };

console.log(target);

(async () => {
    const page = await browser.newPage();
    page.on('console', (msg) => console.log(msg.text()));
});

(async function () {
    const browser = await puppeteer.launch({
        headless: false,
        args: [`--window-size=${viewport.width},${viewport.height}`,
            `--disable-fre`,
            `--no-default-browser-check`,
            `--no-first-run`,
            `--ignore-certificate-errors`]
    });
    const page = await browser.newPage();
    await page.setViewport(viewport);
    // await page.goto('https://en.wikipedia.org/wiki/%22Hello,_World!%22_program');
    await page.goto(target, { waitUntil: 'networkidle2' });
    await page.screenshot({ path: './screenshot.png' });
    browser.close();
})();