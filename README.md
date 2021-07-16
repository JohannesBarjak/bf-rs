# bf-rs
<p align="center">
	<a href="https://opensource.org/licenses/MPL-2.0"><img src="https://img.shields.io/badge/License-MPL%202.0-brightgreen.svg"></a>
</p>
A optimizing brainfuck interpreter and c transpiler written in rust

## Usage
### Transpiler
```shell
bf /path/to/file

gcc -O2 file.c
./a.out
```

### Interpreter
```shell
bf -i /path/to/file
```

## Installation

### on linux
```shell
git clone https://github.com/h4x0r-droid/bf-rs.git

cd bf-rs

cargo build --release
cp ./target/release/bf ~/.local/bin
```

## Implementation details
* The  memory tape is 180,000 cells
* The pointer starts in the 90,000th cell
* Cells are 8bits in size