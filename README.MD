# Sudoku Solver and generator
A general sudoku Solver and generator written in rust
## Basic usage
Building:
```sh
cargo build
```
After building you can solve a 9x9 (with 3x3 inner block) using
```sh
./target/debug/sudoku-rs solve 3 3 < sample/1.sudoku
```
You can also generate a 9x9  sudoku using
```sh
./target/debug/sudoku-rs gen 3 3 100
```
If you're using 100 it would try to remove as much number as posible, if you don't want it remove any number use 0.

## Speed Concern
The debug build is quite fast but I recommend you use release build for better speed.
```sh
cargo build --release
```
Then use
```sh
./target/release/sudoku-rs
```
Even for small sized sudoku like 16x16 (with 4x4 inner block) it take significant amount of time to try to remove all the number. So although this program in theory can generate larger size, it's not meant to be used for larger sudoku.
