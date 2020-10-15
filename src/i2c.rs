use cortex_m;
use tm4c129x;

//use board;
use board::{SDA2, SCL2, I2C_DIV};

pub fn i2c2_master_init(){
    cortex_m::interrupt::free(|_cs| {
        let sysctl = unsafe { &*tm4c129x::SYSCTL::ptr() };
                
        sysctl.rcgcgpio.modify(|_, w| { w.r12().bit(true) }); //port N (I2C mod 2)
        while !sysctl.prgpio.read().r12().bit() {} //port N
        
        sysctl.rcgci2c.modify(|_, w| w.r2().bit(true));
        while !sysctl.pri2c.read().r2().bit() {} //I2C module 2 Peripheral Ready ?

        let gpio_n = unsafe { &*tm4c129x::GPIO_PORTN::ptr() };
        gpio_n.odr.write(|w| w.ode().bits(SDA2)); //SDA (PN4) - open drain (1)
        gpio_n.dir.write(|w| w.dir().bits(SDA2|SCL2)); //SDA (PN4) - open drain (1)
        gpio_n.den.write(|w| w.den().bits(SDA2|SCL2));    
        gpio_n.afsel.write(|w| w.afsel().bits(SDA2|SCL2)); //alternate function select for PN4 and PN5
        gpio_n.pctl.write(|w| unsafe { w.pmc4().bits(3).pmc5().bits(3) });
        
        let i2c_2 = unsafe { &*tm4c129x::I2C2::ptr() };
        i2c_2.mcr.write(|w| w.mfe().bit(true)); //master mode is enabled
        i2c_2.mtpr.write(|w| unsafe { w.tpr().bits(I2C_DIV as u8)}); //set timer
        //i2c_2.mtpr.write(|w| unsafe { w.tpr().bits(0x3B)}); //set timer
        
        //i2c_2.mcs.write(|w| w.mcs().start().bit(true));
    });    
}

pub fn i2c_transmit(master_data:u8, slave_addr: u8) {
    cortex_m::interrupt::free(|_cs| {
        let i2c_2 = unsafe { &*tm4c129x::I2C2::ptr() };
        i2c_2.msa.write(|w| unsafe {w
            .sa().bits(slave_addr)
            .rs().clear_bit()
        });
  
        while i2c_2.mcs.read().busbsy().bit() {} // check if the bus is busy
  
        i2c_2.mdr.write(|w| unsafe {w.data().bits(master_data)});
        i2c_2.mcs.write(|w|unsafe {w.bits(0b0000_0111)}); 
        
        //wait until BUSY is set, this part is crucial for the performance of I2C master, when it transmits
        //we wait until transmission begins
        while !i2c_2.mcs.read().busy().bit() {}         
        while i2c_2.mcs.read().busy().bit() {} //wait until BUSY is low, i.e., trammission is ended
        
    });
}

pub fn i2c_transmit_2bytes(byte1:u8, byte2:u8, slave_addr: u8){
    
    //cortex_m::interrupt::free(|_cs| {
        
        let i2c_2 = unsafe { &*tm4c129x::I2C2::ptr() };
        //let gpio_k = unsafe { &*tm4c129x::GPIO_PORTK::ptr() };                        
        
        i2c_2.msa.write(|w| unsafe {w
            .sa().bits(slave_addr)
            .rs().clear_bit()
        });
                
        while i2c_2.mcs.read().busbsy().bit() {} // check if the bus is busy
  
        i2c_2.mdr.write(|w| unsafe {w.data().bits(byte1)});           
        i2c_2.mcs.write(|w| unsafe {w.bits(0b0000_0011)});
               
        //wait until BUSY is set, this part is crucial for the performance of I2C master, when it transmits
        //we wait until transmission begins
        while !i2c_2.mcs.read().busy().bit() {}         
        while i2c_2.mcs.read().busy().bit() {} //wait until BUSY is low, i.e., trammission is ended                

        i2c_2.mdr.write(|w| unsafe {w.data().bits(byte2)});        
        i2c_2.mcs.write(|w| unsafe {w.bits(0b0000_0101)});       
        
        while !i2c_2.mcs.read().busy().bit() {}
        while i2c_2.mcs.read().busy().bit() {}
        
    //});    
}

pub fn i2c_transmit_3bytes(byte1:u8, byte2:u8, byte3:u8, slave_addr: u8){
    
    cortex_m::interrupt::free(|_cs| {
        
        let i2c_2 = unsafe { &*tm4c129x::I2C2::ptr() };
        //let gpio_k = unsafe { &*tm4c129x::GPIO_PORTK::ptr() };                        
        
        i2c_2.msa.write(|w| unsafe {w
            .sa().bits(slave_addr)
            .rs().clear_bit()
        });
                
        while i2c_2.mcs.read().busbsy().bit() {} // check if the bus is busy
  
        i2c_2.mdr.write(|w| unsafe {w.data().bits(byte1)});           
        i2c_2.mcs.write(|w| unsafe {w.bits(0b0000_0011)});

        //wait until BUSY is set, this part is crucial for the performance of I2C master, when it transmits
        //we wait until transmission begins
        while !i2c_2.mcs.read().busy().bit() {}         
        while i2c_2.mcs.read().busy().bit() {} //wait until BUSY is low, i.e., trammission is ended                

        i2c_2.mdr.write(|w| unsafe {w.data().bits(byte2)});        
        i2c_2.mcs.write(|w| unsafe {w.bits(0b0000_0001)});       
        
        while !i2c_2.mcs.read().busy().bit() {}
        while i2c_2.mcs.read().busy().bit() {}

        i2c_2.mdr.write(|w| unsafe {w.data().bits(byte3)});        
        i2c_2.mcs.write(|w| unsafe {w.bits(0b0000_0101)});       
        
        while !i2c_2.mcs.read().busy().bit() {}
        while i2c_2.mcs.read().busy().bit() {}
        
    });    
}

//This LCD display doesn't have the option of I2C readout
//there is no possibility to check if the function works properly

pub fn i2c_receive(slave_addr: u8)-> u32 {

    let i2c_2 = unsafe { &*tm4c129x::I2C2::ptr() };
              
        i2c_2.msa.write(|w| unsafe {w   
            .sa().bits(slave_addr)         
            .rs().set_bit() //1 - RECEIVE
            //.rs().clear_bit()
        });                

        i2c_2.mcs.write(|w|unsafe {w.bits(0b0000_0111)}); 
        
        while !i2c_2.mcs.read().busy().bit() {}         
        while i2c_2.mcs.read().busy().bit() {}
       
      let  master_data = i2c_2.mdr.read().bits();

    master_data
}