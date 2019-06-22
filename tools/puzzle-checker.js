const utils = require('./utils');

(async () => {
  if (process.argv.length !== 4) {
    console.error("node checker.js puzzle.cond task.dec");
    process.exit(-1);
  }
  const puzzlePath = process.argv[2];
  const taskPath = process.argv[3];
  const result = await utils.checkPuzzle(puzzlePath, taskPath);
  console.log(JSON.stringify(result));

  if (!result.success) {
    process.exit(-1);
  }
})();
