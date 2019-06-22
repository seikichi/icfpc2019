const puppeteer = require('puppeteer');

async function checkSolution(taskPath, solutionPath, boostersPath) {
  const browser = await puppeteer.launch();
  const page = await browser.newPage();
  await page.goto('https://icfpcontest2019.github.io/solution_checker/');
  const task = await page.$('#submit_task');
  const solution = await page.$('#submit_solution');

  await task.uploadFile(taskPath);
  await page.waitForFunction('document.querySelector("#output").innerText === "Done uploading task description"');

  await solution.uploadFile(solutionPath);
  await page.waitForFunction('document.querySelector("#output").innerText === "Done uploading solution"');

  if (boostersPath) {
    const boosters = await page.$('#submit_boosters');
    await boosters.uploadFile(boostersPath);
    await page.waitForFunction('document.querySelector("#output").innerText === "Done uploading solution"');
    // TODO: need to wait? (like checkPuzzle)
  }

  await page.click('#execute_solution');
  await page.waitForFunction('/^(Success|Failed)/.test(document.querySelector("#output").innerText)');
  const output = await page.$eval("#output", output => output.textContent);
  await browser.close();

  const success = output.startsWith('Success');
  const result = { success, timeunits: success ? parseInt(output.match(/[0-9]+/)[0], 10) : null, message: output };
  return result;
}

async function checkPuzzle(puzzlePath, taskPath) {
  const browser = await puppeteer.launch();
  const page = await browser.newPage();
  await page.goto('https://icfpcontest2019.github.io/puzzle_checker/');
  const puzzle = await page.$('#submit_task'); // NOT BUG
  const task = await page.$('#submit_solution'); // NOT BUG

  await puzzle.uploadFile(puzzlePath);
  await task.uploadFile(taskPath);

  while (true) {
    await page.click('#execute_solution');
    const output = await page.$eval('#output', output => output.textContent);
    if (output === 'Validating the puzzle solution...') {
      break;
    }
    await new Promise(resolve => setTimeout(resolve, 100));
  }

  await page.waitForFunction('/^(Success|Failed)/.test(document.querySelector("#output").innerText)');
  const output = await page.$eval("#output", output => output.textContent);
  await browser.close();

  const success = output.startsWith('Success');
  const result = { success, message: output };
  return result;
}

exports.checkSolution = checkSolution;
exports.checkPuzzle = checkPuzzle;
