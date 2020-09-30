//https://rust-embedded.github.io/book/intro/no-std.html
#![no_std] // Removes std from the prelude //
#![no_main]
#![allow(non_snake_case)]


use core::cell::Cell;
use core::fmt;

extern crate cortex_m;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::Peripherals; //re-export from crate::peripheral::Peripherals;
use cortex_m::interrupt::Mutex;

extern crate cortex_m_rt;
use cortex_m_rt::entry; //needed for the use of the entry attribute
use cortex_m_rt::exception; //needed for the use of the exception attribute

#[macro_use(interrupt)]
extern crate tm4c129x;

//const SYS_TICK_RELOAD_VALUE: u32 = 24000; //2.5kHz
//const SYS_TICK_RELOAD_VALUE: u32 = 48000; //1.25kHz
//const SYS_TICK_RELOAD_VALUE: u32 = 120000; //500Hz
//const SYS_TICK_RELOAD_VALUE: u32 = 1200000; //500Hz
//const SYS_TICK_RELOAD_VALUE: u32 = 12_000_000; //5Hz
const SYS_TICK_RELOAD_VALUE: u32 = 15_000_000; //4Hz

static SYS_TICK_IRQ_COUNT: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));

//*create static variables for seconds, minutes, and hours
static SEC_IRQ_COUNT: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
static MIN_IRQ_COUNT: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
static HOURS_IRQ_COUNT: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));

//* static variable to keep ADC values
static IC_SAMPLE: Mutex<Cell<u16>> = Mutex::new(Cell::new(0));
static FBI_SAMPLE: Mutex<Cell<u16>> = Mutex::new(Cell::new(0));
static FV_SAMPLE: Mutex<Cell<u16>> = Mutex::new(Cell::new(0)); 
static FD_SAMPLE: Mutex<Cell<u16>> = Mutex::new(Cell::new(0)); 
static AV_SAMPLE: Mutex<Cell<u16>> = Mutex::new(Cell::new(0)); 
static FBV_SAMPLE: Mutex<Cell<u16>> = Mutex::new(Cell::new(0));

//*device under test - choose ADC channel to test
//const adc_ch_to_test:u8 = 1; //ch IC_sample 
//const adc_ch_to_test:u8 = 2; //ch FBI_sample
//const adc_ch_to_test:u8 = 3; //ch FV_sample
//const adc_ch_to_test:u8 = 4; //ch FD_sample //not used in the current version of IonPak
//const adc_ch_to_test:u8 = 5; //ch AV_sample
const ADC_CH_TO_TEST:u8 = 6; //ch FBV_sample

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        write!($crate::UART0, $($arg)*).unwrap()
    })
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

#[no_mangle] // https://github.com/rust-lang/rust/issues/{38281,51647}
#[panic_handler]
pub fn panic_fmt(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

//connect modules:
mod board;

pub struct UART0;

//implement fmt::Write trait for UART0 structure
impl fmt::Write for UART0 {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        let uart_0 = unsafe { &*tm4c129x::UART0::ptr() };
        for c in s.bytes() {
            while uart_0.fr.read().txff().bit() {}
            uart_0.dr.write(|w| w.data().bits(c))
        }
        Ok(())
    }
}

#[entry] //Attribute to declare the entry point of the program
fn main() -> ! {
    board::init();
    board::start_sys_tick(SYS_TICK_RELOAD_VALUE);
    board::start_adc();
    /*
    let p = Peripherals::take().unwrap();
    let mut syst = p.SYST;

    // set clock source to the Core clock using Core field of the SystClkSource enumeration
    syst.set_clock_source(SystClkSource::Core);
    // set timer to wrap every sys_tick_reload_value ticks 
    syst.set_reload(SYS_TICK_RELOAD_VALUE-1); //timer wraps 4 times per 1s
    syst.enable_counter();
    syst.enable_interrupt();
    */

    loop {     
        /*  
        board::set_led(true);
        board::set_led1(true);
        board::set_led3(true);
        board::set_led(false);       
        board::set_led1(false);       
        board::set_led3(false);       
        */
    }
}

//*description of #[exception] attribute
//https://docs.rs/cortex-m-rt/0.6.12/cortex_m_rt/attr.exception.html
/*
the name of the function must be one of:

    DefaultHandler
    NonMaskableInt
    HardFault
    MemoryManagement (a)
    BusFault (a)
    UsageFault (a)
    SecureFault (b)
    SVCall
    DebugMonitor (a)
    PendSV
    SysTick
*/ 


#[exception] //Attribute to declare an exception handler
fn SysTick() {
    cortex_m::interrupt::free(|cs| {        
        let sys_tick_irq_count = SYS_TICK_IRQ_COUNT.borrow(cs);
        let sec_irq_count = SEC_IRQ_COUNT.borrow(cs);
        let min_irq_count = MIN_IRQ_COUNT.borrow(cs);
        let hours_irq_count = HOURS_IRQ_COUNT.borrow(cs);
        
        sys_tick_irq_count.set(sys_tick_irq_count.get() + 1);        
        if sys_tick_irq_count.get()==4{
             //define ADC variables
             let ic_sample  = IC_SAMPLE.borrow(cs);
             let fbi_sample = FBI_SAMPLE.borrow(cs);
             let fv_sample  = FV_SAMPLE.borrow(cs);
             let fd_sample  = FD_SAMPLE.borrow(cs);
             let av_sample  = AV_SAMPLE.borrow(cs);
             let fbv_sample = FBV_SAMPLE.borrow(cs);
             
             match ADC_CH_TO_TEST {
                 1 => println!("IC {}\r", ic_sample.get()),
                 2 => println!("FBI {}\r", fbi_sample.get()),
                 3 => println!("FV {}\r", fv_sample.get()),
                 4 => println!("FD {}\r", fd_sample.get()),
                 5 => println!("AV {}\r", av_sample.get()),
                 6 => println!("FBV {}\r", fbv_sample.get()),
                 _ => (),
             }

            let mut cp = unsafe { tm4c129x::CorePeripherals::steal() };
            cp.NVIC.enable(tm4c129x::Interrupt::ADC0SS0);

            //LED on Launch development board
            let gpio_n = unsafe { &*tm4c129x::GPIO_PORTN::ptr() };
            gpio_n.data.modify(|r, w| w.data().bits(r.data().bits() ^ board::LED3));
            //IonPak LEDs            
            let gpio_k = unsafe { &*tm4c129x::GPIO_PORTK::ptr() };                        
            gpio_k.data.modify(|r, w| w.data().bits(r.data().bits() ^ board::LED1)); //toggle red LED1
            gpio_k.data.modify(|r, w| w.data().bits(r.data().bits() ^ board::LED2)); //toggle green LED2             
            
            sys_tick_irq_count.set(0);
            sec_irq_count.set(sec_irq_count.get() + 1);
            if sec_irq_count.get() == 59 {
                sec_irq_count.set(0);                
                min_irq_count.set(min_irq_count.get() + 1);
                if min_irq_count.get() == 59 {
                    min_irq_count.set(0);                
                    hours_irq_count.set(hours_irq_count.get() + 1);
                }
            }
            //println!("{:02}:{:02}\r", min_irq_count.get(), sec_irq_count.get());
            //print!(" {:02}:{:02}:{:02}\r", hours_irq_count.get(), min_irq_count.get(), sec_irq_count.get());
        }
        
    })
}

interrupt!(ADC0SS0, adc0_ss0);
fn adc0_ss0() {
    cortex_m::interrupt::free(|cs| {        
        let adc0 = unsafe { &*tm4c129x::ADC0::ptr() };     
        if adc0.ostat.read().ov0().bit() {
            panic!("ADC FIFO overflowed")        
        }
        adc0.isc.write(|w| w.in0().bit(true));    

        //save data into these static variables
        // the FIFO buffer is constantly provided with the data
        // the sampling is continuous adc0.emux.write(|w| w.em0().always()); //continuously sample, p1093
        // it means that the data is provided continuously but it has to be read out immediately as the interrupt is active
        //otherwise the new data will be discarded and oferflow will occur. It is impossible to recover after overflow, this is why 
        //we use panic error handling and the programme is stopped
        // the print out of data is given in SysTick() interrupt routine
        let ic_sample  = IC_SAMPLE.borrow(cs);
        let fbi_sample = FBI_SAMPLE.borrow(cs);
        let fv_sample  = FV_SAMPLE.borrow(cs);
        let fd_sample  = FD_SAMPLE.borrow(cs);
        let av_sample  = AV_SAMPLE.borrow(cs);
        let fbv_sample = FBV_SAMPLE.borrow(cs);
        
        ic_sample.set(adc0.ssfifo0.read().data().bits());
        fbi_sample.set(adc0.ssfifo0.read().data().bits());
        fv_sample.set(adc0.ssfifo0.read().data().bits()); 
        fd_sample.set(adc0.ssfifo0.read().data().bits());
        av_sample.set(adc0.ssfifo0.read().data().bits());
        fbv_sample.set(adc0.ssfifo0.read().data().bits());
        
        //Toggle LED on every entrance in the interrupt routine
        //let gpio_n = unsafe { &*tm4c129x::GPIO_PORTN::ptr() };            
        //gpio_n.data.modify(|r, w| w.data().bits(r.data().bits() ^ 0x02));        
    });
}
