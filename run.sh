#!/bin/bash

INPUT="./measurements.txt"
OUTPUT="./output.txt"

rm -f $OUTPUT

time ./target/release/onebrc $INPUT $OUTPUT
# cargo flamegraph -- $INPUT $OUTPUT

if [ -f $OUTPUT ]; then
    diff --color=always <(./tocsv.sh < ./result.txt) <(./tocsv.sh < $OUTPUT)
else
    echo "No output file"
fi
