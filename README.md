# adc-isr-test

This test has been developed in order to check if physical connections 
between ADC pins and PCB are there. 

The test allows to generate 6 different test files, depending on the 
settings in the constant ADC_CH_TO_TEST. The generated programming
file adc-isr-test has to be renamed to:

- adc-isr-test-ch-AV.out
- adc-isr-test-ch-FBI.out
- adc-isr-test-ch-FBV.out
- adc-isr-test-ch-FD.out
- adc-isr-test-ch-FV.out
- adc-isr-test-ch-IC.out

Some tests are dependent on the setup. THe details are in the power
point presentation IonPak_PCB_debugging.pptx. For example, FBV test
without any connections to this channel will result in the output "FBV 0",
due to the fact that the default bias of this channel is ~ -50mV, it 
means that a proper bias has to be provided. In this particular case, the
transformer TR350 has to be removed.