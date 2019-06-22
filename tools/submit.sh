#!/bin/bash

set -eux

DIR=$(mktemp -d)
pushd $DIR

aws s3 cp --recursive --include '*.sol' 's3://icfpc2019-hanase/solutions/problems/' .
zip solutions.zip *.sol
curl -F private_id=${ICFPC2019_PRIVATE_ID} -F file=@solutions.zip https://monadic-lab.org/submit

popd
rm -rf $DIR
