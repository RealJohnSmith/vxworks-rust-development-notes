Unofficial notes, documentation and knowledge collected when attempting to develop applications in __rust__ for __vxworks__ real-time operating system.


These notes are result of an effort in a university class ([https://rtime.felk.cvut.cz/psr](https://rtime.felk.cvut.cz/psr)) to explore the way how to use __rust__ language in __vxworks__, both in simulator and on real hardware. Feel free to use these notes for your needs. You are most welcome and encouraged to __open a pull request__ if you find some inaccuracies, want to document your own struggles with similar systems or just want to provide more knowledge for the future generations.


---

VxWorks version used in this document: vxworks-22.06

The guide is split into several following sections:

### [Installation - linux](installation-linux.md), including [CLion](https://www.jetbrains.com/clion/) support

Development machine: linux (Debian 5.10.46-4) - x86_64
Simulator: Running on the same (x86_64) machine


### [First Rust project](first-project.md) that runs in the simulator


### [Real hardware](hardware-armv7.md)

Real hardware: University [development board](https://rtime.felk.cvut.cz/psr/cviceni/mzapo/) which is based on [MicroZed board](https://www.avnet.com/wps/portal/us/products/avnet-boards/avnet-board-families/microzed) with [Xilinx Zynq-7000 System on Chip](https://www.xilinx.com/products/silicon-devices/soc/zynq-7000.html). The FPGA chip has integrated ARM core on the same die. We will be programming only the ARM chip. This chip can however interface with the FPGA as an external periphery, if the need arises in your project.

Real hardware compilation target: armv7-wrs-vxworks-eabihf




### [API bindings, snippets, known issues](snippets/README.md)


