# NXP i.MX RT 1060 Evaluation Kit Board Support

Provides the needed setup to create applications on the 1060 evaluation kit.

Most of the examples run now though require a segger jlink to run them. The i2c
and spi examples have not yet been fleshed out and tested yet.

## Examples

To run an example an external JLink should be setup with the EVK board

Then the jlink gdb server may be started with the following

``` sh
./jlink_gdb.sh
```

Afterwards the examples may be run using cargo run (the runner and linker settings are defined in .cargo/config)

``` sh
cargo run --example=led --features=rt
```

Notably this loads the program into sram, executes a small memcpy, then jumps to the appropriate instruction.

## Acknowledgements

Based on the fantastic work of Ian McIntyre to support the teensy 4
[`teensy4-rs`](https://github.com/mciantyre/)
