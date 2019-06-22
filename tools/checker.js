const puppeteer = require('puppeteer');

(async () => {
  const browser = await puppeteer.launch();
  const page = await browser.newPage();
  await page.goto('https://icfpcontest2019.github.io/solution_checker');
  const task = await page.$('#submit_task');
  const solution = await page.$('#submit_solution');

  await task.uploadFile('./tmp/001.sol');
  await task.uploadFile('./tasks/part-1-initial/prob-001.desc');

  await page.click('#execute_solution');
  await page.waitFor(2000);
  console.log(await page.$eval("#output", output => output.textContent));

  await browser.close();
})();
