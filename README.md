# `STM32F3DISCOVERY Quickstart`

> A template for building applications for the STM32F3DISCOVERY development board.

This project is a fork of official cortex-m quickstart project, which is developed and maintained by the [Cortex-M team][team].

## Dependencies

To build embedded programs using this template you'll need:

- Rust 1.31, 1.30-beta, nightly-2018-09-13 or a newer toolchain. e.g. `rustup
  default beta`

- The `cargo generate` subcommand. [Installation
  instructions](https://github.com/ashleygwilliams/cargo-generate#installation).

- `rust-std` components (pre-compiled `core` crate) for the ARM Cortex-M
  targets. Run:

``` console
$ rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf
```

## Using this template

**NOTE**: This is the very short version that only covers building programs. For
the long version, which additionally covers flashing, running and debugging
programs, check [the embedded Rust book][book].

[book]: https://rust-embedded.github.io/book

In this example we'll be using the STM32F3DISCOVERY. This board contains an
STM32F303VCT6 microcontroller. This microcontroller has:

- A Cortex-M4F core that includes a single precision FPU

- 256 KiB of Flash located at address 0x0800_0000.

- 40 KiB of RAM located at address 0x2000_0000. (There's another RAM region but
  for simplicity we'll ignore it).

1. Instantiate the template.

``` console
$ cargo generate --git https://github.com/SailnMobula/stm32f3-discovery-quickstart
 Project Name: first-f3discovery-app
 Creating project called `first-f3discovery-app`...
 Done! New project created /tmp/first-f3discovery-app

$ cd first-f3discovery-app
```

1. The compilation target is alread set to STM32F303VCT6 which is `thumbv7em-none-eabihf` in the `.cargo/config`.

``` console
$ tail -n9 .cargo/config.toml
```

``` toml
[build]
# Pick ONE of these compilation targets
# target = "thumbv6m-none-eabi"    # Cortex-M0 and Cortex-M0+
# target = "thumbv7m-none-eabi"    # Cortex-M3
# target = "thumbv7em-none-eabi"   # Cortex-M4 and Cortex-M7 (no FPU)
target = "thumbv7em-none-eabihf" # Cortex-M4F and Cortex-M7F (with FPU)
# target = "thumbv8m.base-none-eabi"   # Cortex-M23
# target = "thumbv8m.main-none-eabi"   # Cortex-M33 (no FPU)
# target = "thumbv8m.main-none-eabihf" # Cortex-M33 (with FPU)
```

1. The correct memory region is already set in the `memory.x` file.

``` console
$ cat memory.x
/* Linker script for the STM32F303VCT6 */
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x08000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 40K
}
```

4. Build the template application or one of the examples.

``` console
$ cargo build
```

## VS Code

This template includes launch configurations for debugging CortexM programs with Visual Studio Code located in the `.vscode/` directory.  
See [.vscode/README.md](./.vscode/README.md) for more information.  
If you're not using VS Code, you can safely delete the directory from the generated project.

# License

This template is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], the maintainer of this crate, the [Cortex-M team][team], promises
to intervene to uphold that code of conduct.

[CoC]: https://www.rust-lang.org/policies/code-of-conduct
[team]: https://github.com/rust-embedded/wg#the-cortex-m-team
