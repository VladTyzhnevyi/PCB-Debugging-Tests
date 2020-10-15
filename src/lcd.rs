use i2c;
use board;

use board::delay_us;

const line1_base:u8 = 0x00;
const line2_base:u8 = 0x40;

pub const LINE1:u8 = 0;
pub const LINE2:u8 = 1;

pub const DISP_ON_OFF_D_POS:u8 = 2;
pub const DISP_ON_OFF_C_POS:u8 = 1;
pub const DISP_ON_OFF_B_POS:u8 = 0;

pub fn LCD_init(slave_addr: u8){
    cortex_m::interrupt::free(|_cs| {
        
        let i2c_2 = unsafe { &*tm4c129x::I2C2::ptr() };
       // let gpio_k = unsafe { &*tm4c129x::GPIO_PORTK::ptr() };                        
        
        i2c_2.msa.write(|w| unsafe {w
            .sa().bits(slave_addr)
            .rs().clear_bit()
        });
                
        while i2c_2.mcs.read().busbsy().bit() {} // check if the bus is busy
  
        //function set, DL=1, N=1
        //i2c_transmit_2bytes(0x00, 0x38, slave_addr);
        
        
        i2c_2.mdr.write(|w| unsafe {w.data().bits(0x00)});           
        i2c_2.mcs.write(|w| unsafe {w.bits(0b0000_0011)});

        //wait until BUSY is set, this part is crucial for the performance of I2C master, when it transmits
        //we wait until transmission begins
        while !i2c_2.mcs.read().busy().bit() {}         
        while i2c_2.mcs.read().busy().bit() {} //wait until BUSY is low, i.e., trammission is ended                
        
        i2c_2.mdr.write(|w| unsafe {w.data().bits(0x38)});        
        i2c_2.mcs.write(|w| unsafe {w.bits(0b0000_0001)});       
                
        while !i2c_2.mcs.read().busy().bit() {}
        while i2c_2.mcs.read().busy().bit() {}

        delay_us(10000);

        //i2c_2.mdr.write(|w| unsafe {w.data().bits(0x00)});           
        //i2c_2.mcs.write(|w| unsafe {w.bits(0b0000_0011)});

        //wait until BUSY is set, this part is crucial for the performance of I2C master, when it transmits
        //we wait until transmission begins
        //while !i2c_2.mcs.read().busy().bit() {}         
        //while i2c_2.mcs.read().busy().bit() {} //wait until BUSY is low, i.e., trammission is ended                

        i2c_2.mdr.write(|w| unsafe {w.data().bits(0x39)});        
        i2c_2.mcs.write(|w| unsafe {w.bits(0b0000_0001)});       
                
        while !i2c_2.mcs.read().busy().bit() {}
        while i2c_2.mcs.read().busy().bit() {}

        /*
        for i in 0..100000 {
            count = count +1;
            gpio_k.data.modify(|r, w| w.data().bits(r.data().bits() ^ LED1)); //toggle red LED1
        }
        */
        delay_us(5000);
        
        //internal OSC frequency
        i2c_2.mdr.write(|w| unsafe {w.data().bits(0x14)});        
        i2c_2.mcs.write(|w| unsafe {w.bits(0b0000_0001)});  //frame freq is 131 Hz (2 line mode) VDD=3.0V     
        
        while !i2c_2.mcs.read().busy().bit() {}
        while i2c_2.mcs.read().busy().bit() {}
        /*
        for i in 0..1000 {
            count = count +1;
            gpio_k.data.modify(|r, w| w.data().bits(r.data().bits() ^ LED1)); //toggle red LED1
        }*/

        delay_us(50);

        //contrast set
        i2c_2.mdr.write(|w| unsafe {w.data().bits(0x7F)});        
        //i2c_2.mdr.write(|w| unsafe {w.data().bits(0x78)}); //display starts showing something        
        //i2c_2.mdr.write(|w| unsafe {w.data().bits(0x7A)}); //0x7A - acceptable quality       
        i2c_2.mcs.write(|w| unsafe {w.bits(0b0000_0001)});       
        
        while !i2c_2.mcs.read().busy().bit() {}
        while i2c_2.mcs.read().busy().bit() {}
        
        delay_us(50);

        //Power/ICON control/contrast set
        //i2c_2.mdr.write(|w| unsafe {w.data().bits(0x5E)}); //C5 =1 , LCD has all cells ON - too much contrast
        //i2c_2.mdr.write(|w| unsafe {w.data().bits(0x5D)}); //C4 =1 , LCD has all cells ON - too much contrast
        i2c_2.mdr.write(|w| unsafe {w.data().bits(0x5C)}); //BON=1, C5=0, C4 = 0;
        i2c_2.mcs.write(|w| unsafe {w.bits(0b0000_0001)});       
        
        while !i2c_2.mcs.read().busy().bit() {}
        while i2c_2.mcs.read().busy().bit() {}
        
        delay_us(50);
       
        //Follower control
        i2c_2.mdr.write(|w| unsafe {w.data().bits(0x6E)}); //0x6E is optimal, the display works only with this value        
        //i2c_2.mdr.write(|w| unsafe {w.data().bits(0x6D)});        
        i2c_2.mcs.write(|w| unsafe {w.bits(0b0000_0001)});       
        
        while !i2c_2.mcs.read().busy().bit() {}
        while i2c_2.mcs.read().busy().bit() {}
        
        delay_us(50);
    
        //Display ON/OFF            
        //i2c_2.mdr.write(|w| unsafe {w.data().bits(0x0C)});        
        i2c_2.mdr.write(|w| unsafe {w.data().bits(0x0D)}); //cursor on
        i2c_2.mcs.write(|w| unsafe {w.bits(0b0000_0001)});       
        
        while !i2c_2.mcs.read().busy().bit() {}
        while i2c_2.mcs.read().busy().bit() {}
        
        delay_us(50);
        
        //Clear Display
        i2c_2.mdr.write(|w| unsafe {w.data().bits(0x01)});        
        i2c_2.mcs.write(|w| unsafe {w.bits(0b0000_0001)});       
        
        while !i2c_2.mcs.read().busy().bit() {}
        while i2c_2.mcs.read().busy().bit() {}
        
        delay_us(2000);
    
        i2c_2.mdr.write(|w| unsafe {w.data().bits(0x06)});        
        i2c_2.mcs.write(|w| unsafe {w.bits(0b0000_0101)});       
        
        while !i2c_2.mcs.read().busy().bit() {}
        while i2c_2.mcs.read().busy().bit() {}
          
    });  
}

pub fn set_cursor(line:u8, position:u8){
    if line==0 && (position>=0) && (position<20) {
        i2c::i2c_transmit_2bytes(0x00, 0x80|line1_base|position, board::DISP_I2C_ADDR);
        delay_us(30);
    }else 
    if line==1 && (position>=0) && (position<20) {
        i2c::i2c_transmit_2bytes(0x00, 0x80|line2_base|position, board::DISP_I2C_ADDR);
        delay_us(30);
    }    
}

pub fn disp_cursor_on_off(d_mask:u8, c_mask:u8, b_mask:u8){
    i2c::i2c_transmit_2bytes(0x00, 0x08|d_mask|c_mask|b_mask, board::DISP_I2C_ADDR);
    delay_us(30);
}

pub fn disp_clear(){
    i2c::i2c_transmit_2bytes(0x00, 0x01, board::DISP_I2C_ADDR);
    delay_us(1000);
}