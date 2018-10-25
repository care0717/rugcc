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

runtest 'main() { return 128; }' 128
runtest 'main() { return 2+3; }' 5
runtest 'main() { return 10-3; }' 7
runtest 'main() { return 5+20-4-2; }' 19
runtest 'main() { return 12 +  34 - 5; }' 41
runtest 'main() { return 1+2+3+4+5+6+7+8+9+10+11+12+13; }' 91
runtest 'main() { return 2*3; }' 6
runtest 'main() { return 10/3+1; }' 4
runtest 'main() { return (2+3)*(4+5); }' 45
runtest 'main() { a=2; return a; }' 2
runtest 'main() { a=2+6/2; a=a*2; return a; }' 8
runtest 'main() { a=2; b=5+1; return a*b; }' 12
runtest 'main() { if (1) return 1+2; return 3*(1+3); }' 3
runtest 'main() { if (0) return 1+2; return 3*(1+3); }' 12
runtest 'main() { a=1; if (a) a=2; else a=3; return a; }' 2
runtest 'main() { a=0; if (a) a=2; else a=3; return a; }' 3

runtest 'main() { return _plus(2, 3); }' 5
runtest 'one() { return 1; } main() { return one(); }' 1
runtest 'one() { return 1; } two() { return 2; } main() { return one()+two(); }' 3

runtest 'mul(a, b) { return a * b; } main() { return mul(2, 3); }' 6
runtest 'add(a,b,c,d,e,f) { return a+b+c+d+e+f; } main() { return add(1,2,3,4,5,6); }' 21
runtest 'sum(a) { if (a) return a+sum(a-1); return 0; } main() { return sum(10); }' 55

echo "OK"