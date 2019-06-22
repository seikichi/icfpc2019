const utils = require('./utils');

const fs = require('fs');
const S3 = require('aws-sdk/clients/s3');
const tmp = require('tmp');

const s3 = new S3({ params: { Bucket: 'icfpc2019-hanase' }});

async function upload(key, solutionPath) {
  const data = fs.readFileSync(solutionPath);
  return s3.putObject({ Key: key, Body: data }).promise();
}

function timeunits(solution) {
  const actions = solution.split('#')[0];
  return actions.match(/[A-Z]/g).length;
}

async function upload_with_check(key, taskPath, solutionPath) {
  const current = await utils.checkSolution(taskPath, solutionPath);
  if (!current.success) {
    console.error(`Invalid task or solution ${key}, ${taskPath}, ${solutionPath}`);
    process.exit(-1);
  }
  upload(key, solutionPath);
}

(async () => {
  if (process.argv.length !== 5) {
    console.error("node upload.js id task.desc solution.sol");
    process.exit(-1);
  }

  const problemId = process.argv[2];
  const taskPath = process.argv[3];
  const solutionPath = process.argv[4];

  const key = `solutions/problems/prob-${problemId}.sol`;
  try {
    await s3.headObject({ Key: key }).promise();
  } catch (error) {
    if (error.code === 'NotFound') {
      console.log(`${key} does not exists, try to upload the given solution.`);
      upload_with_check(key, taskPath, solutionPath);
      return;
    }
    throw error;
  }

  const object = await s3.getObject({ Key: key }).promise();
  const old_timeunits = timeunits(object.Body.toString());
  const new_timeunits = timeunits(fs.readFileSync(solutionPath, 'utf-8'));

  if (old_timeunits <= new_timeunits) {
    console.log(`The new solution of problem ${problemId} (${new_timeunits}) does not better than old one (${old_timeunits}).`);
    return;
  }

  console.log(`The new solution of problem ${problemId} (${new_timeunits}) seems better than old one (${old_timeunits}), try to upload it.`);
  upload_with_check(key, taskPath, solutionPath);
})();
