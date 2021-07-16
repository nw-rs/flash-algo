# NumWorks n0110 flash algorithm

This is a flash algorithm for the STM32F730V8T6 and Adesto AT25SF641 QSPI flash chip, used in the NumWorks n0110 calculator. 
It implements the CMSIS-Pack ABI, so it's compatible with any tools that use it, including probe-rs.

## Dependencies

Run the following requirements:
```bash
cargo install cargo-binutils && rustup component add llvm-tools-preview rust-src
```
## Building

Building requires nightly Rust.

Just run `build.sh`. It spits out the flash algo in the probe-rs YAML format:

    rsworks-flash-algo$ ./build.sh 
    instructions: sLVD9jgAGUzE8gIAAWh8REHwAgEBYAEgQPIBISBwQfIAAMryAADA8hYBQWABIcDyADEBYKsgACEA8Ir4RPJAJcDyDwUA8Mj4AT370VAgACEA8H74MSABIQElAPB5+DggACEA8HX4ACAlcLC9ygEAAARJeUQIeAEoEr8BIAAgCHBwRwC/ZgEAAIC1CEh4RAB4ASgcvwEggL0GIADwcvjHIADwb/gA8H34ACCAvUwBAACwtQRGDUh4RAB4ASgQ0QYgAPBf+EHyCAVC9thwyvIABehgLGEoaIAGBNUA8H34+ecBILC9APBd+AAgsL0iAQAA8LWBsAZGFUh4RAB4ASgg0QYgFEYNRgDwPPjtsUHyCAdoHsryAAe4YEL2AnDA8gAw+GA+YRT4AQsBPTh2+tEB4ADwUvg4aIAG+tQA8DL4ACAA4AEgAbDwvf7eAL/gAAAAELVB8ggEACLK8gAEomAhsQIhYWFP9IJBAeBP9IBxwLIIQ+BgIGiABli/EL0A8Cz4+OcQtUHyCAQDIcryAARh8x8g4GAgaIAGWL8QvQDwHPj45/C1gbBB8ggEQPIFNsryAAQAJcDyAHalYOZgp2kgaIAGAtUA8Aj4+ef4BwLQAPAD+PHnAbDwvQC/cEcA1NTU
    pc_init: 1
    pc_uninit: 113
    pc_program_page: 241
    pc_erase_sector: 177
    pc_erase_all: 137

## Hacking

The `algo` module contains the FlashAlgo trait, and an `algo!` macro to generate
the glue functions for a given struct implementing it. This is generic for all chips, so feel free to reuse it!

`main.rs` has the actual implementation for NumWorks calculator.

# License

This thingy is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
