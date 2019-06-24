#!/bin/bash

set -eux

ID=$1
TMP=$(mktemp)

./target/release/main_beam < tmp/problems/prob-${ID}.desc > $TMP;
node tools/upload.js $ID tmp/problems/prob-${ID}.desc $TMP;


