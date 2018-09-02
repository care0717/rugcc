#!/bin/bash

runtest() {
    echo "$1" | cargo run > tmp.s
    cc  -o tmp.exe tmp.s

    ./tmp.exe
    out=$?
    if [ "$out" != "$2" ]; then
        echo "$1: $2 expected. but got $out"
        exit 1
    fi
    echo "$1 => $2"
    rm -f tmp.*
}

runtest 0 0
runtest 1 1
runtest 128 128
runtest '5+20-4' 21
runtest ' 12 +  34 - 5 ' 41

echo "OK"