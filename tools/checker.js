const puppeteer = require('puppeteer');

(async () => {
  if (process.argv.length !== 4) {
    console.error("node checker.js task.desc solution.sol");
    process.exit(-1);
  }
  const taskPath = process.argv[2];
  const solutionPath = process.argv[3];

  const browser = await puppeteer.launch();
  const page = await browser.newPage();
  await page.goto('https://icfpcontest2019.github.io/solution_checker');
  const task = await page.$('#submit_task');
  const solution = await page.$('#submit_solution');

  await task.uploadFile(taskPath);
  await page.waitForFunction('document.querySelector("#output").innerText === "Done uploading task description"');

  await solution.uploadFile(solutionPath);
  await page.waitForFunction('document.querySelector("#output").innerText === "Done uploading solution"');

  await page.click('#execute_solution');
  await page.waitForFunction('/^(Success|Failed)/.test(document.querySelector("#output").innerText)');
  const result = await page.$eval("#output", output => output.textContent);
  console.log(result);

  await browser.close();

  if (!result.startsWith('Success')) {
    process.exit(-1);
  }
})();
