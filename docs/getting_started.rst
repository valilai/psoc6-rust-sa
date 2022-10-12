#############
Rust on PSoC6
#############

Before we write the space rocket game for the infineon psoc6 prototyping kit we want to 
run a simple hello world program with Rust and run it on the psoc6.
In the next tutorial we will discuss how to install mbed os on the board and build an
developer environment with vs code and platformIO to build rust programs. 
The last tutorial will explain how the space rocket game is coded in rust. 

os
---
For our first test we will program rust for bare-metal and not for mbedos as planed in 
this project. 

Rust bare-metal
---------------
In a bare metal environment no code has benn loaded before your program. So there is no 
os on the board where we can compile our program. We have to cross-compile it on a host
machine and load it to the board. 

setup host machine 
---------------
For cross-compilation we are using ubuntu 22.04.1 LTS on a Lenovo ThinkPad T14s Gen 1. 

1. install rustup:
  ``curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh``
  ``source ~/.profile``
  ``source ~/.bashrc``
  rustup is an installer for the system programming language Rust

2. setup "Nightly Rust":
   ``restup default nightly``
   New Rust versions are realesed in a train schedule. The night channel is
   updated every night and has the newest features but also some bugs. 

3. install cargo subcommands:
   ``cargo install cargo-clone``
   ``cargo install cargo-edit``
   ``cargo install cargo-binutils``

4. install OpenOCD
   ``sudo apt-get install openocd``

5. install the rust-std component for your target
   ``rustup target add thumbv7em-none-eabi`` 
   thumbv7em-none-eabi = Cortex M4 (no fpu)
   psoc6 = Cortex M4 & M0+ -> we only use the M4 now

6. install sublime text for coding 
   ``sudo wget -O- https://download.sublimetext.com/sublimehq-pub.gpg | gpg --dearmor | sudo tee /usr/share/keyrings/sublimehq.gpg``
   ``echo 'deb [signed-by=/usr/share/keyrings/sublimehq.gpg] https://download.sublimetext.com/ apt/stable/' | sudo tee /etc/apt/sources.list.d/sublime-text.list``
   ``sudo apt update``
   ``sudo apt install sublime-text`` 
   --> use sublime text with ``subl`` command. 


Build hello world program
-------------------------
For our first hello world program we will use the hal & pac from https://github.com/psoc-rs
written by mvertescher & jonas-schievink 

1. build cargo application
   ``cargo new rust-hello``

2. open application project with editor
   .. code-block:: shell

        cd rust-hello
        subl .

3. edit cargo configuration file Cargo.toml
   .. code-block:: yaml
        
        panic-halt = "0.2.0"
        [dependencies.psoc6-hal]
        git = "https://github.com/psoc-rs/psoc6-hal"
        rev = "86684f0" 

4. update application project
   ``cargo update``

5. build program
   ``cargo build``

Create psoc6 HL
--------------
For our first hello world program we will use a sample program
from embedded-rust that can be cloned with:
``cargo clone cortex-m-quickstart``  

We now have to adapt the memory layout to our board in memory.x
Change: 
``FLASH : ORIGIN = 0x000BAAD0, LENGTH = 0K
  RAM : ORIGIN = 0xBAAD0000, LENGTH = 0K``
to 
``FLASH : ORIGIN = 0x10000000, LENGTH = 256K
  RAM :   ORIGIN = 0x08002000, LENGTH = 32K``

an uncomment
``_stack_start = ORIGIN(RAM) + LENGTH(RAM);``

you can find the memory layout in the psoc6 data sheet: https://www.cypress.com/file/385921/download

Next we set a default build target in .cargo/config with: 
``[build]
  target = "thumbv7em-none-eabihf"`` 


