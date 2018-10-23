#!/bin/bash

runtest() {
    cargo build
    ./target/debug/rugcc "$1" > tmp.s
    echo 'int plus(int x, int y) { return x + y; }' | gcc -xc -c -o tmp-plus.o -
    cc  -o tmp.exe tmp.s tmp-plus.o

    ./tmp.exe
    out=$?
    if [ "$out" != "$2" ]; then
        echo "$1: $2 expected. but got $out"
        rm -f tmp*
        exit 1
    fi
    echo "$1 => $2"
    rm -f tmp*
}

runtest 'return 128;' 128
runtest 'return 2+3;' 5
runtest 'return 10-3;' 7
runtest 'return 5+20-4-2;' 19
runtest 'return 12 +  34 - 5;' 41
runtest 'return 1+2+3+4+5+6+7+8+9+10+11+12+13;' 91
runtest 'return 2*3;' 6
runtest 'return 10/3+1;' 4
runtest 'return (2+3)*(4+5);' 45
runtest 'a=2; return a;' 2
runtest 'a=2+6/2; a=a*2; return a;' 8
runtest 'a=2; b=5+1; return a*b;' 12
runtest 'if (1) return 1+2; return 3*(1+3);' 3
runtest 'if (0) return 1+2; return 3*(1+3);' 12
runtest 'if (1) a=2; else a=3; return a;' 2
runtest 'if (0) a=2; else a=3; return a;' 3


runtest 'return plus(2, 3);' 5

echo "OK"