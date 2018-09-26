#!/bin/bash

runtest() {
    echo "$1" | cargo run > tmp.s
    cc  -o tmp.exe tmp.s

    ./tmp.exe
    out=$?
    if [ "$out" != "$2" ]; then
        echo "$1: $2 expected. but got $out"
        rm -f tmp.*
        exit 1
    fi
    echo "$1 => $2"
    rm -f tmp.*
}

runtest 'return 1;' 1
runtest 'return 128;' 128
runtest 'return 2+3;' 5
runtest 'return 10-3;' 7
runtest 'return 5+20-4-2;' 19
runtest 'return 12 +  34 - 5;' 41
runtest 'return 1+2+3+4+5+6+7+8+9+10+11+12+13;' 91
runtest 'return 2*3;' 6
runtest 'return 10/3;' 3

echo "OK"