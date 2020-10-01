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

//#[macro_use(interrupt)]
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
    
    let p = Peripherals::take().unwrap();
    let mut syst = p.SYST;

    // set clock source to the Core clock using Core field of the SystClkSource enumeration
    syst.set_clock_source(SystClkSource::Core);
    // set timer to wrap every sys_tick_reload_value ticks 
    syst.set_reload(SYS_TICK_RELOAD_VALUE-1); //timer wraps 4 times per 1s
    syst.enable_counter();
    syst.enable_interrupt();

    let duty:u16 = 10; //PWM period is 1200 clock ticks
    board::set_fbv_pwm(duty);        
    board::set_hv_pwm(duty);
    board::set_fv_pwm(duty);
    println!("FBV, FV, HV duty cycle = {}\r", duty);

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
            print!(" {:02}:{:02}:{:02}\r", hours_irq_count.get(), min_irq_count.get(), sec_irq_count.get());
        }
        
    })
}