# Real hardware

Real hardware: University [development board](https://rtime.felk.cvut.cz/psr/cviceni/mzapo/) which is based on [MicroZed board](https://www.avnet.com/wps/portal/us/products/avnet-boards/avnet-board-families/microzed) with [Xilinx Zynq-7000 System on Chip](https://www.xilinx.com/products/silicon-devices/soc/zynq-7000.html). 
Real hardware compilation target: armv7-wrs-vxworks-eabihf

*Paths (which may include version numbers in directory names) used in this tutorial are taken from the university lab setup. Your paths/versions may differ.*

*Tested on machine: linux (Debian 5.10.46-4) - x86_64*

---

## 1. VxWorks Source Build (VSB) project

We need a VSB project to build upon. Luckily for us, there is one already provided* in `/opt/psr/zynq7k-base`, preconfigured to cross-compile to `armv7-wrs-vxworks-eabihf` target and includes `BUNDLE_RUST` and `INCLUDE_POSIX_PTHREAD_SCHEDULER`.

* *One provided is for the university course; it is out of scope of this guide how to create it* 

We will use this project (or rather its fork) and put our rust code to `usr/rust/` inside this project, where we can run `cargo new application-name`


## 2. Compile

Don't forget to compile the project to produce the output files - `cargo build`

## 3. Uploading to the board

Open your fork of this VSB project in WindRiver IDE, connect the board and in the file tree locate file `usr/rust/application-name/target/armv7-wrs-vxworks-eabihf/debug/application-name.vxe`

Right click on this file and select "Build and launch". This will start a build process that takes about one to two minutes.

__TODO__ screenshots

I have experienced a lot of errors during this build, related to ssl libraries. Luckily I didn't need to use it and it was possible to launch the program anyways.

*Anyone is welcome to figure out the source of this issue and then open a PR to update this documentation.*


After updating your code, don't forget to repeat both steps, `2.` and `3.`
