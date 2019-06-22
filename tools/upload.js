const utils = require('./utils');

const fs = require('fs');
const S3 = require('aws-sdk/clients/s3');
const tmp = require('tmp');

const s3 = new S3({ params: { Bucket: 'icfpc2019-hanase' }});

async function upload(key, solutionPath) {
  const data = fs.readFileSync(solutionPath);
  return s3.putObject({ Key: key, Body: data }).promise();
}

(async () => {
  if (process.argv.length !== 5) {
    console.error("node upload.js id task.desc solution.sol");
    process.exit(-1);
  }

  const problemId = process.argv[2];
  const taskPath = process.argv[3];
  const solutionPath = process.argv[4];

  const current = await utils.check(taskPath, solutionPath);
  if (!current.success) {
    console.error('invalid task or solution');
    process.exit(-1);
  }

  const key = `solutions/problems/prob-${problemId}.desc`;
  try {
    await s3.headObject({ Key: key }).promise();
  } catch (error) {
    if (error.code === 'NotFound') {
      console.log(`${key} does not exists, upload the given solution.`);
      upload(key, solutionPath);
      return;
    }
    throw error;
  }
  const object = await s3.getObject({ Key: key }).promise();

  const tmpFile = tmp.fileSync();
  fs.writeFileSync(tmpFile.name, object.Body);

  const prev = await utils.check(taskPath, tmpFile.name);
  if (!prev.success) {
    console.error(`ERROR: invalid solution in the S3 bucket!!! (key = ${key})`);
    process.exit(-1);
  }

  if (current.timeunits < prev.timeunits) { // TODO: <= -> <
    console.log(`The given solution (${current.timeunits}) seems better than old one (${prev.timeunits}), upload it ... (key = ${key})`);
    upload(key, solutionPath);
  }

  tmpFile.removeCallback();
})();
