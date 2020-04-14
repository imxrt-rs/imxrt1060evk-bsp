# EVK1060 Examples

All examples can be built with a single command

```sh
cargo build 
```

Running them requires a JLinkGDBServer running and connected to the 1060EVK
rather than the built in opensda debugger. The examples run directly from RAM
and are not written to flash. In the future the https://github.com/imxrt-rs/imxrt-rt-gen
along with https://github.com/imxrt-rs/imxrt-boot-gen will provide the necessary
linker scripts, Reset handler, and flash configuration array required to run
images from flash, setting up SDRAM, ITCM/DTCM, and perhaps XIP related features.


To run an example

Start the JLinkGDBServer, a script is here to help starting it with
the right arguments.

```
./jlink_gdb.sh
```

Followed by

```
cargo run --bin pit
```

Note the --bin arg may be any of the examples in this directory.

On a linux machine with arm gdb installed should leave you at the Reset
handler waiting for input. If this does not happen please file an issue
with steps done so this README may be improved.
