const utils = require('./utils');

const path = require('path');
const fs = require('fs');
const fetch = require('node-fetch');

const publicId = '99';

const wrapper = '/home/ec2-user/src/icfpc2019/target/release/main_cloning';

(async () => {
  while (true) {
    console.log("Sleep 10 seconds ...");
    await new Promise(resolve => setTimeout(resolve, 10 * 1000));

    try {
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
        console.log(`getblockinfo request failed, ${JSON.stringify(json)}`);
        continue;
      }

      const {
        block,
        puzzle,
        task,
        excluded,
      } = json.result;
      console.log(`block = ${block}`);

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

      // T.B.D.
      // const solutionResult = await utils.checkSolution(inTaskPath, outSolutionPath);
      // if (!solutionResult.success) {
      //   console.log(`Solution Chcker Failed: ${JSON.stringify(solutionResult)}`);
      //   continue;
      // }

      // const puzzleResult = await utils.checkPuzzle(inPuzzlePath, outTaskPath);
      // if (!puzzleResult.success) {
      //   console.log(`Puzzle Chcker Failed: ${JSON.stringify(puzzleResult)}`);
      //   continue;
      // }

      const params = [
        block,
        path.resolve(outSolutionPath),
        path.resolve(outTaskPath),
      ];

      const submitResponse = await fetch('http://127.0.0.1:8332/', {
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
      const submitJson = await submitResponse.json();
      if (submitJson.error || (submitJson.result && submitJson.result.errors)) {
        console.log(`submit request failed, ${JSON.stringify(submitJson)}`);
        continue;
      }
      console.log(`Done: ${JSON.stringify(submitJson)}`);
    } catch (e) {
      console.log(e);
    }
  }
})();
