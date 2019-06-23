const utils = require('./utils');

const exec = require('child_process').exec;
const path = require('path');
const fs = require('fs');
const fetch = require('node-fetch');

const publicId = '99';
const wrapper = path.resolve('target/release/main_cloning');
const puzzler = path.resolve('target/release/puzzler');

const webhookUrl = process.env['ICFPC2019_SLACK_WEBHOOK_URL'];
if (!webhookUrl) {
  console.error('Please set ICFPC2019_SLACK_WEBHOOK_URL env');
  process.exit(-1);
}

async function getblockinfo() {
  const response = await fetch('http://127.0.0.1:8332/', {
    method: 'POST',
    headers: {
      'Content-Type': 'text/plain',
    },
    body: JSON.stringify({
      id: 'curl',
      jsonrpc: '2.0',
      method: 'getblockinfo',
    }),
  });
  const json = await response.json();
  if (json.error || (json.result && json.result.errors)) {
    throw new Error(`getblockinfo request failed, ${JSON.stringify(json)}`);
  }
  return json;
}

async function submit(params) {
  const response = await fetch('http://127.0.0.1:8332/', {
    method: 'POST',
    headers: {
      'Content-Type': 'text/plain',
    },
    body: JSON.stringify({
      id: 'curl',
      jsonrpc: '2.0',
      method: 'submit',
      params,
    }),
  });
  const json = await response.json();
  if (json.error || (json.result && json.result.errors)) {
    throw new Error(`submit request failed, ${JSON.stringify(json)}`);
  }
  return json;
}

function postMessageToSlack(message) {
  // NOTE: do not return promise (just try to send the message. that's all)
  fetch(webhookUrl, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ text: message }),
  });
}

(async () => {
  let last = 0;
  while (true) {
    console.log("Sleep 10 seconds ...");
    await new Promise(resolve => setTimeout(resolve, 10 * 1000));

    try {
      const { block, puzzle, task, excluded } = (await getblockinfo()).result;
      console.log(`block = ${block}`);

      if (last === block) {
        console.log(`This block is already submitted, skip.`);
        continue;
      }

      if (excluded.indexOf(publicId) !== -1) {
        console.log(`blockinfo.excluded contains ${publicId}, skip.`);
        continue;
      }

      const inPuzzlePath = `in-${block}.cond`;
      const inTaskPath = `in-${block}.desc`;
      const outSolutionPath = `out-${block}.sol`;
      const outTaskPath = `out-${block}.desc`;

      fs.writeFileSync(inPuzzlePath, puzzle);
      fs.writeFileSync(inTaskPath, task);

      await new Promise((resolve, reject) => {
        exec(`${wrapper} < ${inTaskPath} > ${outSolutionPath}`, error => {
          if (error) {
            reject(error);
            return;
          }
          resolve();
        });
      });
      await new Promise((resolve, reject) => {
        exec(`${puzzler} < ${inPuzzlePath} > ${outTaskPath}`, error => {
          if (error) {
            reject(error);
            return;
          }
          resolve();
        });
      });

      const solutionResult = await utils.checkSolution(inTaskPath, outSolutionPath);
      if (!solutionResult.success) {
        console.log(`Solution Chcker Failed: ${JSON.stringify(solutionResult)}`);
        continue;
      }

      const puzzleResult = await utils.checkPuzzle(inPuzzlePath, outTaskPath);
      if (!puzzleResult.success) {
        console.log(`Puzzle Chcker Failed: ${JSON.stringify(puzzleResult)}`);
        continue;
      }
      console.log("LGTM, let's submit!");

      const result = await submit([
        block,
        path.resolve(outSolutionPath),
        path.resolve(outTaskPath),
      ]);

      console.log(`Done: ${JSON.stringify(result)}`);
      postMessageToSlack(`Submit Done: block = ${block}, timeunits = ${solutionResult.timeunits}, result = ${JSON.stringify(result)}`);
      last = block;
    } catch (e) {
      console.log(e);
      postMessageToSlack(`mining.js: Error ${e}`);
    }
  }
})();
