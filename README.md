# hackerman

An assembler for 16-bit Hack machine language written in Rust.

![hackerman logo](https://raw.githubusercontent.com/onatm/hackerman/master/assets/hackerman.png)

## Instructions

### Build and test

```bash
$ cargo build
$ cargo test
```

### Run

```bash
$ cargo run --release /path/to/input.asm /path/to/output.hack
```

## Example Hack Assembly Program

_Note: You can learn about Hack machine language specification from [here](https://docs.wixstatic.com/ugd/44046b_7ef1c00a714c46768f08c459a6cab45a.pdf)._

#### Rect.asm

```x86
// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/06/rect/Rect.asm

// Draws a rectangle at the top-left corner of the screen.
   @0
   D=M
   @INFINITE_LOOP
   D;JLE
   @counter
   M=D
   @SCREEN
   D=A
   @address
   M=D
(LOOP)
   @address
   A=M
   M=-1
   @address
   D=M
   @32
   D=D+A
   @address
   M=D
   @counter
   MD=M-1
   @LOOP
   D;JGT
(INFINITE_LOOP)
   @INFINITE_LOOP
   0;JMP
```

#### Rect.hack

```binary
0000000000000000
1111110000010000
0000000000010111
1110001100000110
0000000000010000
1110001100001000
0100000000000000
1110110000010000
0000000000010001
1110001100001000
0000000000010001
1111110000100000
1110111010001000
0000000000010001
1111110000010000
0000000000100000
1110000010010000
0000000000010001
1110001100001000
0000000000010000
1111110010011000
0000000000001010
1110001100000001
0000000000010111
1110101010000111
```

## License

[MIT](LICENSE)
