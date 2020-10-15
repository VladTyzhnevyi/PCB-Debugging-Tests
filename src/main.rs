//https://rust-embedded.github.io/book/intro/no-std.html
#![no_std] // Removes std from the prelude //
#![no_main]
#![allow(non_snake_case)]


use core::cell::Cell;
use core::fmt;
use core::char;

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
//const SYS_TICK_RELOAD_VALUE: u32 = 6_000_000; //10Hz
const SYS_TICK_RELOAD_VALUE: u32 = 15_000_000; //4Hz

static SYS_TICK_IRQ_COUNT: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));

//*create static variables for seconds, minutes, and hours
static SEC_IRQ_COUNT: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
static MIN_IRQ_COUNT: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
static HOURS_IRQ_COUNT: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));

static NEW_SEC_FLAG: Mutex<Cell<u8>> = Mutex::new(Cell::new(0));
static CONTRAST: Mutex<Cell<u8>> = Mutex::new(Cell::new(0b0011_0000));
static POSITION: Mutex<Cell<u8>> = Mutex::new(Cell::new(19));

static  CURR_LINE: Mutex<Cell<u8>> = Mutex::new(Cell::new(19));
//static mut CURR_LINE:u8 =0x00;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        write!($crate::UART0, $($arg)*).unwrap()
    })
}

#[macro_export]
macro_rules! print_i2c2 {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        write!($crate::I2C2, $($arg)*).unwrap()
    })
}

#[macro_export]
macro_rules! println_i2c2 {
    ($fmt:expr) => (print_i2c2!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print_i2c2!(concat!($fmt, "\n"), $($arg)*));
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
mod i2c;
mod lcd;

use lcd::{LINE1, LINE2};

pub struct UART0;
pub struct I2C2;

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


//implement fmt::Write trait for I2C2

impl fmt::Write for I2C2 {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        let i2c_2 = unsafe { &*tm4c129x::I2C2::ptr() };
        //let curr_line:Mutex<Cell<u8>>;
        //let curr_line = cortex_m::interrupt::free(|cs| {
        //    CURR_LINE.borrow(cs).borrow()});
        //let mut i:u8 = 0;
        for c in s.bytes() {            
            i2c::i2c_transmit_2bytes(0x40, c as u8, board::DISP_I2C_ADDR);
            //unsafe{CURR_LINE = 1};
            //i=i+1;


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
    
    board::delay_us(10000);
    
    lcd::LCD_init(board::DISP_I2C_ADDR);
    
    let d_mask:u8 = 1<<lcd::DISP_ON_OFF_D_POS;
    let c_mask:u8 = 1<<lcd::DISP_ON_OFF_C_POS;
    let b_mask:u8 = 1<<lcd::DISP_ON_OFF_B_POS;
    //lcd::disp_cursor_on_off(d_mask, c_mask, b_mask);
    lcd::disp_cursor_on_off(d_mask, 0x00, 0x00);
    print_i2c2!("test");
    
    //lcd::set_cursor(line2, 0);
    //board::delay_us(30);
    
    //board::i2c_transmit_2bytes(0x00, 0x80, board::DISP_I2C_ADDR);
    //board::delay_us(30);
    //board::i2c_transmit_2bytes(0x40, 0x03, board::DISP_I2C_ADDR);

    syst.enable_interrupt();
    loop {   
            
        cortex_m::interrupt::free(|cs| {        
            let new_sec_flag = NEW_SEC_FLAG.borrow(cs); 
            let i2c_2 = unsafe { &*tm4c129x::I2C2::ptr() };
            let gpio_k = unsafe { &*tm4c129x::GPIO_PORTK::ptr() };                        
            let contrast = CONTRAST.borrow(cs);
            let cursor_pos = POSITION.borrow(cs);            
            let mut mdr:u32 = 0x00;

            let sec_irq_count = SEC_IRQ_COUNT.borrow(cs);
            let min_irq_count = MIN_IRQ_COUNT.borrow(cs);
            let hours_irq_count = HOURS_IRQ_COUNT.borrow(cs);
           
            if new_sec_flag.get() == 1 {
                new_sec_flag.set(0);                
                //i2c::i2c_transmit_2bytes(0x40, contrast.get(), board::DISP_I2C_ADDR);
                //board::delay_us(30);
                lcd::set_cursor(lcd::LINE1, 0);
                print_i2c2!(" {:02}:{:02}:{:02}\r", hours_irq_count.get(), min_irq_count.get(), sec_irq_count.get());

                //let c:char =    unsafe{char::from_u32_unchecked(contrast.get() as u32)};           
                //print_i2c2!("{}", c);
                //i2c::i2c_transmit(0x00, board::DISP_I2C_ADDR);
                //mdr = i2c::i2c_receive(board::DISP_I2C_ADDR);
                //board::delay_us(30);
                //println!("\r{:?}", mdr);           
             }
        });    
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

        let new_sec_flag = NEW_SEC_FLAG.borrow(cs);
        let contrast = CONTRAST.borrow(cs);
        let cursor_pos = POSITION.borrow(cs);
        
        sys_tick_irq_count.set(sys_tick_irq_count.get() + 1);
        //new_sec_flag.set(1);        
        
        if sys_tick_irq_count.get()==4{
            //LED on Launch development board
            let gpio_n = unsafe { &*tm4c129x::GPIO_PORTN::ptr() };
            //gpio_n.data.modify(|r, w| w.data().bits(r.data().bits() ^ board::LED3));
            //gpio_n.data.modify(|r, w| w.data().bits(r.data().bits() ^ board::SDA2));
            //gpio_n.data.modify(|r, w| w.data().bits(r.data().bits() ^ board::SCL2));
            //IonPak LEDs            
            let gpio_k = unsafe { &*tm4c129x::GPIO_PORTK::ptr() };                        
            //gpio_k.data.modify(|r, w| w.data().bits(r.data().bits() ^ board::LED1)); //toggle red LED1
            //gpio_k.data.modify(|r, w| w.data().bits(r.data().bits() ^ board::LED2)); //toggle green LED2             
            
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
            
            new_sec_flag.set(1);
            
            /*
            if contrast.get()==0x8F {
                contrast.set(0x70)    ;
            }else {
                contrast.set(contrast.get()+1);
            }
            
            if cursor_pos.get()>=19 {
                cursor_pos.set(0);
                //lcd::set_cursor(line2, 0);
            }else{
                cursor_pos.set(cursor_pos.get()+1);
            }                        
            */
        }
        
    })
}