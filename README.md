# bf-rs
<p align="center">
	<a href="https://opensource.org/licenses/MPL-2.0"><img src="https://img.shields.io/badge/License-MPL%202.0-brightgreen.svg"></a>
</p>
An optimizing brainfuck interpreter and compiler written in rust

## Installation

```shell
cargo install --git https://github.com/h4x0r-droid/bf-rs.git
```

## Usage

To interpret your bf program, use the -i flag

```shell
bf -i /path/to/file
```

otherwise it will compile to c by default

```shell
bf /path/to/file
gcc -O3 file.c
```