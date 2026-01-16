# killer_tuidoku
## Introduction

**killer_tuidoku** is a terminal based killer sudoku solver written in Rust. Note that, in the current version, occasionally, when given certain impossible problems **or very difficult problems**, the program will hang and become unusable. For solvable problems, this will resolve itself and the program will yield a valid solution. For invalid problems, however, the program will fail to yield anything and remain in its hung state. Manual termination of the program is required until this issue is resolved in a future version.

## Installation
### Dependencies
- [Rust and cargo](https://www.rust-lang.org/tools/install) as the build dependencies

### Build
```
cd /path/to/repo
cargo build --release
```

### Run
```
cd /path/to/repo/target/release
./tuidoku
```
or
```
cd /path/to/repo
cargo run --release
```
to build and run at the same time. The final compiled binary will still be located at `/path/to/repo/target/release`.
