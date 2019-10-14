# RustGB

RustGB is yet another gameboy emulator written in Rust. The goal of the project is to learn as much as possible about Gameboy emulation while also learning Rust.

## Current State

The project connect yet run any games. I am currently working through the opcode that form the bootloader while I experiment with Rust and emulator design.

## Run with debug

To run with debugging tools:

```bash
cargo run -- -d
```

To stop debugging press `Ctrl+c`

## OpCode page

There were a couple of errors in the main GBZ80 opcode table so I'm hosting a modified version. Almost all the work for it was done by [pastraiser](http://www.pastraiser.com/) rather than myself with the original being found [here](http://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html)

The link to my modified version can be found here: [Opcodes](https://www.guydunton.com/rust-gb/).

## Todo

- [x] Clean up debug cli
- [x] Clean up opcodes directory
- [ ] Continue adding support for bootloader commands.
- [ ] Clean the opcode tests
