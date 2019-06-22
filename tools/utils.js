const puppeteer = require('puppeteer');

async function check(taskPath, solutionPath) {
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
  const output = await page.$eval("#output", output => output.textContent);
  await browser.close();

  const success = output.startsWith('Success');
  const result = { success, timeunits: success ? parseInt(output.match(/[0-9]+/)[0], 10) : null };
  return result;
}

exports.check = check;
