const puppeteer = require('puppeteer');
// const delay = require('delay');
const delay = (...args) => import('delay').then(({default: delay}) => delay(...args));
const fs = require('fs');
const { exit } = require('process');
const speedline = require('speedline');

var target = process.argv[2]
var viewport = { width: 1920, height: 1080 };
var dependency_out_filename = "output.json";
var cost_gain_filename = "cost.json";

console.log(target);

var url = target;
var trace_path = "trace.json";

(async () => {
    const page = await browser.newPage();
    page.on('console', (msg) => console.log(msg.text()));
});

(async function () {
    try {

        if (fs.existsSync(trace_path)) {
            fs.unlinkSync(trace_path);
        }
    
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
        await page.setCacheEnabled(false);
        await page.tracing.start({ path: trace_path, screenshots: true });

        // page.on('console', (msg) => console[msg._type]('PAGE LOG:', msg._text));

        await page.goto(url, { waitUntil: 'networkidle0' });
        await delay(1000);

        try {
            await page.tracing.stop();
        } catch (error) {
            console.log(error);
        }

        results = await speedline(trace_path, {include: 'speedIndex'});
        console.log(results.speedIndex);
        fs.writeFileSync('TRACE_OK', results.speedIndex.toString());

        await page.close();
        await browser.close();
        fs.unlinkSync(trace_path);

        process.exit(0);
    } catch (error) {
        console.log(error);
    } finally {
        process.exit(0);
    }

})();