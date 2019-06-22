const utils = require('./utils');

(async () => {
  if (process.argv.length !== 4) {
    console.error("node checker.js task.desc solution.sol");
    process.exit(-1);
  }
  const taskPath = process.argv[2];
  const solutionPath = process.argv[3];
  const result = await utils.check(taskPath, solutionPath);
  console.log(JSON.stringify(result));

  if (!result.success) {
    process.exit(-1);
  }
})();
