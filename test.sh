#!/bin/bash

runtest() {
    ./target/debug/rugcc "$1" > ./tmp.s
    cat <<EOF | gcc -xc -c -o tmp-test.o -
        int plus(int x, int y) { return x + y; }
        int *alloc(int x) {
            static int arr[1];
            arr[0] = x;
            return arr;
        }
EOF
    cc -o ./tmp.exe ./tmp.s ./tmp-test.o
    ./tmp.exe
    out=$?
    if [ "$out" != "$2" ]; then
        echo "$1: $2 expected. but got $out"
        rm -f ./tmp*
        exit 1
    fi
    echo "$1 => $2"
    rm -f ./tmp*
}

cargo build && cargo test
if [ $? != "0" ]; then
    exit 1
fi

runtest 'int main() { return 128; }' 128
runtest 'int main() { return 2+3; }' 5
runtest 'int main() { return 10-3; }' 7
runtest 'int main() { return 5+20-4-2; }' 19
runtest 'int main() { return 12 +  34 - 5; }' 41
runtest 'int main() { return 1+2+3+4+5+6+7+8+9+10+11+12+13; }' 91
runtest 'int main() { return 2*3; }' 6
runtest 'int main() { return 1+10/3; }' 4
runtest 'int main() { return (2+3)*(4+5); }' 45
runtest 'int main() { int a=2+6/2; a=a*2; return a; }' 10
runtest 'int main() { int a; int b; a=2; b=5+1; return a*b; }' 12
runtest 'int main() { if (1) return 1+2; return 3*(1+3); }' 3
runtest 'int main() { if (0) return 1+2; return 3*(1+3); }' 12
runtest 'int main() { int a=1; if (a) a=2; else a=3; return a; }' 2
runtest 'int main() { int a=0; if (a) a=2; else a=3; return a; }' 3

runtest 'int main() { return plus(2, 3); }' 5
runtest 'int one() { return 1; } int main() { return one(); }' 1
runtest 'int one() { return 1; } int two() { return 2; } int main() { return one()+two(); }' 3

runtest 'int mul(int a, int b) { return a * b; } int main() { return mul(2, 3); }' 6
runtest 'int add(int a,int b,int c,int d,int e,int f) { return a+b+c+d+e+f; } int main() { return add(1,2,3,4,5,6); }' 21
runtest 'int sum(int a) { if (a) return a+sum(a-1); return 0; } int main() { return sum(10); }' 55

runtest 'int main() { return 0||0; }' 0
runtest 'int main() { return 1||0; }' 1
runtest 'int main() { return 0||1; }' 1
runtest 'int main() { return 1||1; }' 1
runtest 'int main() { return 0&&0; }' 0
runtest 'int main() { return 1&&0; }' 0
runtest 'int main() { return 0&&1; }' 0
runtest 'int main() { return 1&&1; }' 1

runtest 'int main() { return 0<0; }' 0
runtest 'int main() { return 1<0; }' 0
runtest 'int main() { return 0<1; }' 1
runtest 'int main() { return 0>0; }' 0
runtest 'int main() { return 0>1; }' 0
runtest 'int main() { return 1>0; }' 1

runtest 'int main() { int sum=0; for (int i=10; i<15; i=i+1) sum = sum + i; return sum;}' 60
runtest 'int main() { int i=1; int j=1; int k; int m; for (k=0; k<10; k=k+1) { m=i+j; i=j; j=m; } return i;}' 89

runtest 'int main() { int *p = alloc(42); return *p; }' 42

echo "OK"
