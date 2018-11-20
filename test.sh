#!/bin/bash

runtest() {
    ./target/debug/rugcc "$1" > ./tmp.s
    cat <<EOF | gcc -xc -c -o tmp-test.o -
        int plus(int x, int y) { return x + y; }
        int *alloc1(int x, int y) {
          static int arr[2];
          arr[0] = x;
          arr[1] = y;
          return arr;
        }
        int *alloc2(int x, int y) {
          static int arr[2];
          arr[0] = x;
          arr[1] = y;
          return arr + 1;
        }
        int **alloc_ptr_ptr(int x) {
          static int **p;
          static int *q;
          static int r;
          r = x;
          q = &r;
          p = &q;
          return p;
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

runtest 'int main() { int *p = alloc1(3,5); return *p + *(1 + p); }' 8
runtest 'int main() { int *p = alloc2(2,7); return *p + *(p - 1); }' 9
runtest 'int main() { int **p = alloc_ptr_ptr(2); return **p; }' 2
runtest 'int main() { int ary[3]; *ary=2; *(ary+1)=4; *(ary+2)=6; return *ary + *(ary+1) + *(ary+2);}' 12
runtest 'int main() { int x; int *p = &x; x = 5; return *p+p[0];}' 10
runtest 'int main() { int ary[2]; ary[0]=1; ary[1]=2; return ary[0] + ary[1];}' 3

runtest 'int main() { char x; return sizeof x; }' 1
runtest 'int main() { int x; return sizeof(x);}' 4
runtest 'int main() { int *x; return sizeof x;}' 8
runtest 'int main() { int ary[4]; return sizeof ary;}' 16

runtest 'int main() { char x = 5; return x; }' 5
runtest 'int main() { int x = 0; char *p = &x; p[0] = 42; return x; }' 42

echo "OK"
