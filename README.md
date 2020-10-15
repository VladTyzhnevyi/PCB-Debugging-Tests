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
4. Backlight LED needs 3.0V or 3.3V
