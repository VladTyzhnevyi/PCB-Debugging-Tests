# i2c and LCD-test

This is the test that programs MIDAS (MCCOG22005A6W-SPTLYI) LCD display. The test results
in initialization of the display, then the word "test" is printed out and in 1 second
starts timer.


## Connections
It is very important to make proper connections. The settings of the display depend on
the VDD, 3V and 3.3V voltage have different register settings.

1. We used the schematic for VDD=3.0V from the **Display_specification.pdf** file 
2. There are 2 capacitors (1uF). One is connected between pin 4 (VDD) and  pin 1 (VOUT).
Another is connected between pins 2 and 3 (CAP1N and CAP1P correspondingly)
3. Pin 5 - ground, Pin 6 - SDA, Pin 7 - SCl. Pin 8 - RST **this pin has to be pulled up** 
the display won't work otherwise

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
