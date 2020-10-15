use cortex_m;
//use cortex_m;
use tm4c129x;

use i2c;
use i2c::i2c2_master_init;

pub const LED1: u8 = 0b0100_0000; // PK6 //red LED
pub const LED2: u8 = 0x10; // PK4 //green LED

pub const  LED3: u8 = 0x01; //PN0

const HV_PWM: u8 = 0x01;  // PF0
const FV_PWM: u8 = 0x04;  // PF2
const FBV_PWM: u8 = 0x01; // PD5

pub const SDA2:u8 = 0b0001_0000; //PN4
pub const SCL2:u8 = 0b0010_0000; //PN5

pub const SDA0:u8 = 0b0000_0100; //PB2
pub const SCL0:u8 = 0b0000_1000; //PB3

/*
const FD_ADC: u8 = 0x01;  // PE0
const FV_ADC: u8 = 0x02;  // PE1
const FBI_ADC: u8 = 0x04; // PE2
const IC_ADC: u8 = 0x08;  // PE3
const FBV_ADC: u8 = 0x20; // PD5
const AV_ADC: u8 = 0x40;  // PD6

const FV_ERRN: u8 = 0x01;    // PL0
const FBV_ERRN: u8 = 0x02;   // PL1
const FBI_ERRN: u8 = 0x04;   // PL2
const AV_ERRN: u8 = 0x08;    // PL3
const AI_ERRN: u8 = 0x10;    // PL4
const ERR_LATCHN: u8 = 0x20; // PL5
const BTNN: u8 = 0x80;       // PL7
const ERR_RESN: u8 = 0x01;   // PQ0
*/
const PWM_LOAD: u16 = (/*pwmclk*/120_000_000u32 / /*freq*/100_000) as u16; //PWM period is 1200 clock ticks
const UART_DIV: u32 = (((/*sysclk*/120_000_000 * 8) / /*baud*/115200) + 1) / 2;
//const I2C_FREQ:u32 = 100000;
//const I2C_FREQ:u32 = 200000;
const I2C_FREQ:u32 = 200000;
pub const I2C_DIV: u32 = ((/*sysclk*/120_000_000 ) / /*baud*/(2*(6+4)*I2C_FREQ) - 1);

//pub const DISP_I2C_ADDR: u8 = 00111110;
pub const DISP_I2C_ADDR: u8 = 0b00111100;
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

pub fn set_hv_pwm(duty: u16) {
    cortex_m::interrupt::free(|_cs| {
        let pwm0 = unsafe { &*tm4c129x::PWM0::ptr() };
        pwm0._0_cmpa.write(|w| w.compa().bits(duty));
    });
}

pub fn set_fv_pwm(duty: u16) {
    cortex_m::interrupt::free(|_cs| {
        let pwm0 = unsafe { &*tm4c129x::PWM0::ptr() };
        pwm0._1_cmpa.write(|w| w.compa().bits(duty));
    });
}

//* set FBV PWM frequency, the duty cycle is set in clock ticks
pub fn set_fbv_pwm(duty: u16) {
    cortex_m::interrupt::free(|_cs| {
        let pwm0 = unsafe { &*tm4c129x::PWM0::ptr() };
        pwm0._2_cmpa.write(|w| w.compa().bits(duty));
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
             .r12().bit(true) //port N (I2C mod 2)
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
        //gpio_n.dir.write(|w| w.dir().bits(LED3|SDA2|SCL2));
        //gpio_n.den.write(|w| w.den().bits(LED3|SDA2|SCL2));
        gpio_n.dir.write(|w| w.dir().bits(LED3|SDA2|SCL2));
        gpio_n.den.write(|w| w.den().bits(LED3|SDA2|SCL2));

        // Set up I2C
        i2c::i2c2_master_init();
        //gpio_k.data.modify(|r, w| w.data().bits(r.data().bits() | LED1)); //clear red LED1
        //i2c0_slave_init();

        


        // Set up PWMs
        let gpio_f = unsafe { &*tm4c129x::GPIO_PORTF_AHB::ptr() };
        gpio_f.dir.write(|w| w.dir().bits(HV_PWM|FV_PWM));
        gpio_f.den.write(|w| w.den().bits(HV_PWM|FV_PWM));
        gpio_f.afsel.write(|w| w.afsel().bits(HV_PWM|FV_PWM));
        gpio_f.pctl.write(|w| unsafe { w.pmc0().bits(6).pmc2().bits(6) });

        let gpio_g = unsafe { &*tm4c129x::GPIO_PORTG_AHB::ptr() };
        gpio_g.dir.write(|w| w.dir().bits(FBV_PWM));
        gpio_g.den.write(|w| w.den().bits(FBV_PWM));
        gpio_g.afsel.write(|w| w.afsel().bits(FBV_PWM));
        gpio_g.pctl.write(|w| unsafe { w.pmc0().bits(6) });

        sysctl.rcgcpwm.modify(|_, w| w.r0().bit(true));
        while !sysctl.prpwm.read().r0().bit() {}

        let pwm0 = unsafe { &*tm4c129x::PWM0::ptr() };
        // HV_PWM
        pwm0._0_gena.write(|w| w.actload().zero().actcmpad().one());
        pwm0._0_load.write(|w| w.load().bits(PWM_LOAD)); //defines period of the PWM signal
        pwm0._0_cmpa.write(|w| w.compa().bits(0));
        pwm0._0_ctl.write(|w| w.enable().bit(true));
        // FV_PWM
        pwm0._1_gena.write(|w| w.actload().zero().actcmpad().one());
        pwm0._1_load.write(|w| w.load().bits(PWM_LOAD));
        pwm0._1_cmpa.write(|w| w.compa().bits(0));
        pwm0._1_ctl.write(|w| w.enable().bit(true));
        // FBV_PWM
        pwm0._2_gena.write(|w| w.actload().zero().actcmpad().one());
        pwm0._2_load.write(|w| w.load().bits(PWM_LOAD));
        pwm0._2_cmpa.write(|w| w.compa().bits(0));
        pwm0._2_ctl.write(|w| w.enable().bit(true));
        // Enable all at once
        pwm0.enable.write(|w| {
            w.pwm0en().bit(true)
             .pwm2en().bit(true)
             .pwm4en().bit(true)
        });
    });
}


pub fn delay_us(us: u32){
    let count = us*120;   
    cortex_m::asm::delay(us*120); //for 120_000_000 Hz clk
    /*
    for i in 0..count {
        cortex_m::asm::nop();
    }
    */
}

fn lcd_command(value:u8, slave_addr:u8) {
    i2c::i2c_transmit_2bytes(0x00, value, slave_addr);
}