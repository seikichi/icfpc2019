const utils = require('./utils');

(async () => {
  if (process.argv.length !== 4 && process.argv.length !== 5) {
    console.error("node checker.js task.desc solution.sol [boosters.buy]");
    process.exit(-1);
  }
  const taskPath = process.argv[2];
  const solutionPath = process.argv[3];
  const boostersPath = process.argv.length === 5 ? process.argv[4] : null;
  const result = await utils.checkSolution(taskPath, solutionPath, boostersPath);
  console.log(JSON.stringify(result));

  if (!result.success) {
    process.exit(-1);
  }
})();
