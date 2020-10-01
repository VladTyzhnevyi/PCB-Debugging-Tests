# uart-test

Makes red LED (LED1_A) and green LED of IonPak blinking every second and outputs the timer value via UART to the terminal.
See the details related to the connections in the pdf file in the main folder of the project.

## Settings
- $ rustup override set1.30.1
- $ rustup override setnightly-2018-12-01-x86_64-unknown-linux-gnu
- $ rustup target add thumbv7em-none-eabihf
- $ cargo check
- $ cargo build–release

These settings have to be applied once to the "root" folder, i.e., IonPak. In case there is a doubt that the settings are valid for the current folder, use **$ rustup show**. The current settings are:

installed targets for active toolchain
--------------------------------------

thumbv7em-none-eabihf
x86_64-unknown-linux-gnu

active toolchain
----------------

**nightly-2018-12-01-x86_64-unknown-linux-gnu** (directory override for '/home/vt/ionpak/PCB-Debugging-Tests/master-uart-test')
**rustc 1.32.0-nightly (d09466ceb 2018-11-30)**

They work fine, and the project can be compiled without problems. Important is that the rust compiler is **nightly** and that the target is **thumbv7em-none-eabihf**
