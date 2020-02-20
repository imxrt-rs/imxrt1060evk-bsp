# NXP i.MX RT 1060 Evaluation Kit Board Support

Provides the needed setup to create applications on the 1060 evaluation kit.

**Note this is in progress and does not fully work yet, there are still linker
and runtime initialization issues being debugged. The program does not copy
itself to TCM (addressed at 0x0) as expected**

## Examples

See the README in the examples directory for example usage.

## Acknowledgements

Based on the fantastic work of Ian McIntyre to support the teensy 4
[`teensy4-rs`](https://github.com/mciantyre/)
