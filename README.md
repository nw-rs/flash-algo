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
    instructions: 8LWBsEpPf0Q4eAEoCL8A8JH4Q/YYAE/2JELE8gIAz/b/cgFqCSVC8iAECiZB8AIBAWIBaEHwAgEBYAFoIfACAQFggWlB8B4BgWFA8gBAxPICAAFqIfAPIUHwIGFB9BBhAWJA9gBBxPICAYtYZfMHE4tQmSNKamPzF0JKYkHyAALE8gICE2pl8wsjE2IDaEPyMAWrQyNDA2DQ+AA0AiRk85NDwPgANAtoZvMbYwtgE2hk8wUTE2CDaCtDg2DQ+Ag0Q/RAI8D4CDSIaEDwcGCIYJBoASE5cEDyASFA8DAAkGBB8gAAwPIWAcryAABBYAEhwPIAMQFgqyAAIQDwmfhE8kAlwPIPBQDw1/gBPfvRUCAAIQDwjfgxIAEhASQA8Ij4OCAAIQDwhPg8cAAgAbDwvbICAAAQtQZMfEQgeAEoHL8BIBC9/yAA8Iz4ACAgcBC9gAEAAIC1CEh4RAB4ASgcvwEggL0GIADwfPjHIADwefgA8If4ACCAvWABAACwtQRGDkh4RAB4ASgS0QYgAPBp+EHyCAVM8tgwyvIABcDyAgBsYehgKGiABgTVAPCF+PnnASCwvQDwZfgAILC9NgEAAPC1gbAGRhlIeEQAeAEoKNEGIBRGDUYA8ET4LbNB8ggHaB7K8gAHJUS4YEzyAjDA8gIwfmH4YDhoQAcC1ADwXPj552AcIXioQjl2BEbz0QHgAPBS+DhogAb61ADwMvgAIADgASABsPC9/t4Av/AAAAAQtUHyCAQAIsryAASiYCGxAiFhYU/0gkEB4E/0gHHAsghD4GAgaIAGWL8QvQDwLPj45xC1QfIIBAMhyvIABGHzHyDgYCBogAZYvxC9APAc+Pjn8LWBsEHyCARA8gU2yvIABAAlwPIAdqVg5mCnaSBogAYC1QDwCPj55/gHAtAA8AP48ecBsPC9AL9wRwDU1NQ=
    pc_init: 1
    pc_uninit: 309
    pc_program_page: 449
    pc_erase_sector: 381
    pc_erase_all: 341

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
