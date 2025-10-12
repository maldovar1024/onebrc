#!/bin/bash

INPUT="./measurements.txt"
OUTPUT="./output.txt"

rm -f $OUTPUT

time ./target/release/onebrc $INPUT $OUTPUT

diff --color=always <(./tocsv.sh < ./result.txt) <(./tocsv.sh < $OUTPUT)
