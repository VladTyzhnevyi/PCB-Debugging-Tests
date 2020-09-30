use cortex_m;
use tm4c129x;


pub const LED1: u8 = 0x10; // PK4 //red LED
pub const LED2: u8 = 0x40; // PK6 //green LED

pub const  LED3: u8 = 0x01; //PN0
/*
const HV_PWM: u8 = 0x01;  // PF0
const FV_PWM: u8 = 0x04;  // PF2
const FBV_PWM: u8 = 0x01; // PD5
*/
const FD_ADC: u8 = 0x01;  // PE0
const FV_ADC: u8 = 0x02;  // PE1
const FBI_ADC: u8 = 0x04; // PE2
const IC_ADC: u8 = 0x08;  // PE3
const FBV_ADC: u8 = 0x20; // PD5
const AV_ADC: u8 = 0x40;  // PD6

/*
const FV_ERRN: u8 = 0x01;    // PL0
const FBV_ERRN: u8 = 0x02;   // PL1
const FBI_ERRN: u8 = 0x04;   // PL2
const AV_ERRN: u8 = 0x08;    // PL3
const AI_ERRN: u8 = 0x10;    // PL4
const ERR_LATCHN: u8 = 0x20; // PL5
const BTNN: u8 = 0x80;       // PL7
const ERR_RESN: u8 = 0x01;   // PQ0
*/
//const PWM_LOAD: u16 = (/*pwmclk*/120_000_000u32 / /*freq*/100_000) as u16;
const UART_DIV: u32 = (((/*sysclk*/120_000_000 * 8) / /*baud*/115200) + 1) / 2;
/*

pub const AV_ADC_GAIN: f32 = 6.792703150912105;
pub const FV_ADC_GAIN: f32 = 501.83449105726623;
pub const FBI_ADC_GAIN: f32 = 1333.3333333333333;
pub const FBI_ADC_OFFSET: f32 = 96.0;
pub const FD_ADC_GAIN: f32 = 3111.1111111111104;
pub const FD_ADC_OFFSET: f32 = 96.0;
pub const FBV_ADC_GAIN: f32 = 49.13796058269066;
pub const FBV_PWM_GAIN: f32 = 0.07641071428571428;
pub const IC_ADC_GAIN_LOW: f32 = 1333333333333.3333;
pub const IC_ADC_GAIN_MED: f32 = 13201320132.0132;
pub const IC_ADC_GAIN_HIGH: f32 = 133320001.3332;
pub const IC_ADC_OFFSET: f32 = 96.0;

pub const FBI_R223: f32 = 200.0;
pub const FBI_R224: f32 = 39.0;
pub const FBI_R225: f32 = 22000.0;
*/

pub fn set_led(state: bool) {
    cortex_m::interrupt::free(|_cs| {
        let gpio_k = unsafe { &*tm4c129x::GPIO_PORTK::ptr() };
        if state {
            gpio_k.data.modify(|r, w| w.data().bits(r.data().bits() | LED2))
        } else {
            gpio_k.data.modify(|r, w| w.data().bits(r.data().bits() & !LED2))
        }
    });
}

pub fn set_led1(state: bool) {
    cortex_m::interrupt::free(|_cs| {
        let gpio_k = unsafe { &*tm4c129x::GPIO_PORTK::ptr() };
        if state {
            gpio_k.data.modify(|r, w| w.data().bits(r.data().bits() | LED1))
        } else {
            gpio_k.data.modify(|r, w| w.data().bits(r.data().bits() & !LED1))
        }
    });
}

pub fn set_led3(state: bool) {
    cortex_m::interrupt::free(|_cs| {
        let gpio_n = unsafe { &*tm4c129x::GPIO_PORTN::ptr() };
        if state {
            gpio_n.data.modify(|r, w| w.data().bits(r.data().bits() | LED3))
        } else {
            gpio_n.data.modify(|r, w| w.data().bits(r.data().bits() & !LED3))
        }
    });
}


pub fn init() {
    cortex_m::interrupt::free(|_cs| {
        let sysctl = unsafe { &*tm4c129x::SYSCTL::ptr() };

        // Set up main oscillator
        sysctl.moscctl.write(|w| w.noxtal().bit(false));
        sysctl.moscctl.modify(|_, w| w.pwrdn().bit(false).oscrng().bit(true));

        // Prepare flash for the high-freq clk
        sysctl.memtim0.write(|w| unsafe { w.bits(0x01950195u32) });
        sysctl.rsclkcfg.write(|w| unsafe { w.bits(0x80000000u32) });

        // Set up PLL with fVCO=480 MHz
        sysctl.pllfreq1.write(|w| w.q().bits(0).n().bits(4));
        sysctl.pllfreq0.write(|w| w.mint().bits(96).pllpwr().bit(true));
        sysctl.rsclkcfg.modify(|_, w| w.pllsrc().mosc().newfreq().bit(true));
        while !sysctl.pllstat.read().lock().bit() {}

        // Switch to PLL (sysclk=120MHz)
        sysctl.rsclkcfg.write(|w| unsafe { w.bits(0b1_0_0_1_0011_0000_0000000000_0000000011) });


        /*
        For power-savings purposes, the peripheral-specific RCGCx, SCGCx, and DCGCx registers (for
        example, RCGCWD) control the clock-gating logic for that peripheral or block in the system while
        the microcontroller is in Run, Sleep, and Deep-Sleep mode, respectively. These registers are located
        in the System Control register map starting at offsets 0x600, 0x700, and 0x800, respectively.

        The RCGCGPIO register provides software the capability to enable and disable GPIO modules in
        Run mode. When enabled, a module is provided a clock and accesses to module registers are
        allowed. When disabled, the clock is disabled to save power and accesses to module registers
        generate a bus fault.

        p. 382
        */
        // Bring up GPIO ports A, D, E, F, G, K, L, P, Q
        sysctl.rcgcgpio.modify(|_, w| {
            w.r0().bit(true)
             .r3().bit(true)
             .r4().bit(true)
             .r5().bit(true)
             .r6().bit(true)
             .r9().bit(true) //port K, LED1, LED2
             .r10().bit(true)
             .r12().bit(true) //port N
             .r13().bit(true)
             .r14().bit(true)
        });
        while !sysctl.prgpio.read().r0().bit() {}
        while !sysctl.prgpio.read().r3().bit() {}
        while !sysctl.prgpio.read().r4().bit() {}
        while !sysctl.prgpio.read().r5().bit() {}
        while !sysctl.prgpio.read().r6().bit() {}
        while !sysctl.prgpio.read().r9().bit() {}
        while !sysctl.prgpio.read().r10().bit() {}
        while !sysctl.prgpio.read().r12().bit() {} //port N
        while !sysctl.prgpio.read().r13().bit() {}
        while !sysctl.prgpio.read().r14().bit() {}

        // Set up UART0
        let gpio_a = unsafe { &*tm4c129x::GPIO_PORTA_AHB::ptr() };
        gpio_a.dir.write(|w| w.dir().bits(0b11));
        gpio_a.den.write(|w| w.den().bits(0b11));
        gpio_a.afsel.write(|w| w.afsel().bits(0b11));
        gpio_a.pctl.write(|w| unsafe { w.pmc0().bits(1).pmc1().bits(1) });

        sysctl.rcgcuart.modify(|_, w| w.r0().bit(true));
        while !sysctl.pruart.read().r0().bit() {}

        let uart_0 = unsafe { &*tm4c129x::UART0::ptr() };
        uart_0.cc.write(|w| w.cs().sysclk());
        uart_0.ibrd.write(|w| w.divint().bits((UART_DIV / 64) as u16));
        uart_0.fbrd.write(|w| w.divfrac().bits((UART_DIV % 64) as u8));
        uart_0.lcrh.write(|w| w.wlen()._8().fen().bit(true));
        uart_0.ctl.write(|w| w.rxe().bit(true).txe().bit(true).uarten().bit(true));

        // Set up LEDs
        let gpio_k = unsafe { &*tm4c129x::GPIO_PORTK::ptr() };
        gpio_k.dir.write(|w| w.dir().bits(LED1|LED2));
        gpio_k.den.write(|w| w.den().bits(LED1|LED2));

        let gpio_n = unsafe { &*tm4c129x::GPIO_PORTN::ptr() };
        gpio_n.dir.write(|w| w.dir().bits(LED3));
        gpio_n.den.write(|w| w.den().bits(LED3));
        
    });
}

pub fn start_adc() {
    cortex_m::interrupt::free(|_cs| {
        let sysctl = unsafe { &*tm4c129x::SYSCTL::ptr() };

        let gpio_d = unsafe { &*tm4c129x::GPIO_PORTD_AHB::ptr() };
        let gpio_e = unsafe { &*tm4c129x::GPIO_PORTE_AHB::ptr() };
        
        gpio_d.afsel.write(|w| w.afsel().bits(FBV_ADC|AV_ADC)); //afsel - GPIO Alternate Function Select
        gpio_d.amsel.write(|w| w.amsel().bits(FBV_ADC|AV_ADC)); //amsel - GPIO Analog Mode Select
        gpio_e.afsel.write(|w| w.afsel().bits(FD_ADC|FV_ADC|FBI_ADC|IC_ADC));
        gpio_e.amsel.write(|w| w.amsel().bits(FD_ADC|FV_ADC|FBI_ADC|IC_ADC));
        
        //rcgcadc - Analog-to-Digital Converter Run Mode Clock Gating Control
        //pradc - Analog-to-Digital Converter Peripheral Ready    
        sysctl.rcgcadc.modify(|_, w| w.r0().bit(true));
        while !sysctl.pradc.read().r0().bit() {}

        let adc0 = unsafe { &*tm4c129x::ADC0::ptr() };
        // VCO 480 / 15 = 32MHz ADC clock
        //adc0.cc.write(|w| w.cs().syspll().clkdiv().bits(15-1));
        
        // VCO 480 / 30 = 16MHz ADC clock
        //adc0.cc.write(|w| w.cs().syspll().clkdiv().bits(30-1));
        
        // VCO 480 / 60 = 8MHz ADC clock
        adc0.cc.write(|w| w.cs().syspll().clkdiv().bits(60-1));
        
        // VCO 480 / 120 = 4MHz ADC clock
        //adc0.cc.write(|w| w.cs().syspll().clkdiv().bits(120-1));
        
        adc0.im.write(|w| w.mask0().bit(true));
        adc0.emux.write(|w| w.em0().always()); //continuously sample, p1093
        adc0.ssmux0.write(|w| {
            w.mux0().bits(0) // IC_ADC
             .mux1().bits(1) // FBI_ADC
             .mux2().bits(2) // FV_ADC
             .mux3().bits(3) // FD_ADC
             .mux4().bits(5) // AV_ADC
             .mux5().bits(6) // FBV_ADC
        });

        //ie5 - 6th Sample Interrupt Enable, p. 1111
        //The raw interrupt signal (INR0 bit) is asserted at the end of the
        //sixth sample's conversion. If the MASK0 bit in the ADCIM register
        //is set, the interrupt is promoted to the interrupt controller.
        
        //end5 - 6th Sample is End of Sequence

        adc0.ssctl0.write(|w| w.ie5().bit(true).end5().bit(true));
        adc0.sstsh0.write(|w| {
            w.tsh0()._4()
             .tsh1()._4()
             .tsh2()._4()
             .tsh3()._4()
             .tsh4()._4()
             .tsh5()._4()
        });        
        
        adc0.sac.write(|w| w.avg()._64x()); //sac - ADC Sample Averaging Control
        adc0.ctl.write(|w| w.vref().bit(true)); //ctl - ADC Control
        adc0.actss.write(|w| w.asen0().bit(true)); //actss - ADC Active Sample Sequencer         

        //enable ADC0SS0 interrupt
        let mut cp = unsafe { tm4c129x::CorePeripherals::steal() };
        cp.NVIC.enable(tm4c129x::Interrupt::ADC0SS0);
        
       /* 
        if tm4c129x::NVIC::is_enabled(tm4c129x::Interrupt::ADC0SS0){
            println!("ADC0 interrupt is enabled\r");    
        }
        */
    });
}

pub fn start_sys_tick(reload_value: u32) {
    let p = cortex_m::Peripherals::take().unwrap();
    let mut syst = p.SYST;

    // configures the system timer to trigger a SysTick exception every second
    syst.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
    //syst.set_reload(12_000_000); // period = 1s
    syst.set_reload(reload_value-1); // period = 1s    
    syst.enable_counter();
    syst.enable_interrupt();  
}