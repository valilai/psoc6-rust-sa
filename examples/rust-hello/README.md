# rust-hello
This is a first hello world program to test cargo build, flash and debug. <br/>
This projest is WIP ! <br/>

# getting-started
To build and flash this program follow these steps: <br/>
1. open a terminal *
1. `cargo build`
2. `cd openocd/scripts`
3. `sudo ../bin/openocd -f interface/cmsis-dap.cfg -f target/psoc6_2m.cfg`
4. open a new terminal (don't close openocd!!!)
5. `echo 'set auto-load safe-path /' >> ~/.gdbinit`
6. `arm-none-eabi-gdb target/thumbv6m-none-eabi/debug/rust-hello` *
7. `(gdb) target extended-remote :3333`
8. `(gdb) load`
9. `(gdb) monitor arm semihosting enable`
10. `(gdb) break main`
11. `(gdb) continue`
12. `(gdb) step`
13. `(gdb) next`
14. Now the message "Hello, world! Rust" shuld be printed on the openocd console

* make sure you are in this directory

# PsoC 6
This program is build for the cortex M0+ of the PsoC. If you want to build it for the cortex M4F than change the target line in .cargo/config.toml

# author
Valentin Lairich <valentin.lairich@gmail.com> <br/>
version = "0.1.0" <br/>
edition = "Oct.19.2022"
