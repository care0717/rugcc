# rugcc
[![CircleCI](https://circleci.com/gh/care0717/rugcc.svg?style=svg)](https://circleci.com/gh/care0717/rugcc)

C compiler made by Rust

## How to use
```$xslt
# Build compiler. It is generated in ./target/debug/rugcc.
cargo build
# Please enter: ./target/debug/rugcc <your code>. It generates assembly.
./target/debug/rugcc 'int main() { int a; int b; a=2; b=5+1; return a*b; }'
```
Please see example in `test.sh`

## refarence
https://github.com/rui314/9cc
