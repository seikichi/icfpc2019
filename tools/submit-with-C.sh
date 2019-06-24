#!/bin/bash

set -eux

COINS=$1
CLONE=$(($COINS / 2000))

echo "Buy ${CLONE} C boosters (from ${COINS} coins)"


DIR_C0=$(mktemp -d)
DIR_C1=$(mktemp -d)
aws s3 cp --recursive --include '*.sol' 's3://icfpc2019-hanase/solutions/problems/' $DIR_C0
aws s3 cp --recursive --include '*.sol' 's3://icfpc2019-hanase/solutions/problems-with-C/' $DIR_C1

DIR=$(mktemp -d)
pushd $DIR

cp $DIR_C0/*.sol .
for i in $(seq 2 $((CLONE + 1))) ; do
    ID=$(printf "%03d" $i)
    echo C > prob-${ID}.buy
    cp ${DIR_C1}/prob-${ID}.sol .
done

zip solutions.zip *.sol *.buy
curl -F private_id=${ICFPC2019_PRIVATE_ID} -F file=@solutions.zip https://monadic-lab.org/submit

popd
