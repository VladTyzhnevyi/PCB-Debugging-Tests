# FBV-FV-HV-pwm-test

This test has been developed in order to check if physical connections 
between **PWM** pins and PCB are there. 

The programming test file **fbv-fv-hv-pwm-test-10.out** allows to generate
3 PWM outputs, that have a **_10_** clock ticks duty cycle. 

## Setup

Remove both transformers in order to avoid accidental generation of anode or cathode bias. The output of flyback converters
depends on the error signal as well, so in order to not to check error signal, we just removed the transformers.

### Note

At the very beginning of the testing make a visual inspection of the PWM pins, whether they are connected or not. More expanded explanantion can be found in the *.pptx file