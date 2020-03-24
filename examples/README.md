# EVK1060 Examples

**Note this is in progress and does not fully work yet, there is still a linker script
issue as the program does not copy itself to ITCM as expected**


All examples can be built with a single command

```sh
cargo build 
```

However this does not necessarily build a .bin you can easily upload using
the built on mass storage interface which is typically mounted as RT1060-EVK.

To build a bin you need to use cargo objcopy

For example to build a .bin for the led blinky example

```
cargo objcopy --bin led -- -O binary led.bin
```

This bin can be copied to the mounted RT1060-EVK drive and will flash
the on board external spi. A hardware reset might be required still.

Debugging so far is easiest to do using an external JTAG debugger like
a jlink. To do so remove the jumpers J47 and J48.

Doing so allows loading and debugging the built elf or bin files using something
like Ozone which can provide a very nice overview of all registers on the device.
