#!/bin/bash

BOOSTERS=$1

set -eux

while :; do
    if [ -n "${BOOSTERS}" ]; then
        echo "BOOSTER EXISTS"
        j=$(date '+%F_%T');
        mkdir tmp/$j-${BOOSTERS};
        for i in {001..300}; do
            echo $j-${BOOSTERS}-$i;
            ./target/release/main_cloning -b ${BOOSTERS} < tmp/problems/prob-${i}.desc > tmp/$j-${BOOSTERS}/prob-${i}.sol;
            node tools/upload.js $i tmp/problems/prob-${i}.desc tmp/$j-${BOOSTERS}/prob-${i}.sol ${BOOSTERS};
        done;
    else
        echo "NO BOOSTER"
        j=$(date '+%F_%T');
        mkdir tmp/$j;
        for i in {001..300}; do
            echo $j-$i;
            ./target/release/main_cloning < tmp/problems/prob-${i}.desc > tmp/$j/prob-${i}.sol;
            node tools/upload.js $i tmp/problems/prob-${i}.desc tmp/$j/prob-${i}.sol
        done;
    fi
done
