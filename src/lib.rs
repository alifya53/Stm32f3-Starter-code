#![no_main]
#![no_std]

use panic_itm;
use cortex_m;
use cortex_m_rt::{entry};
use f3::hal::stm32f30x;
use f3::hal::prelude::*;
use f3::hal::{delay::Delay};
use f3::led::Leds;



#[entry]
fn main()->!{
    // Initialize Peripherals 
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();
    // Initialize Flash & RCC
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    // Initialize Clock
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(cp.SYST,clocks);

   
    
    loop{
        //  Your code here..
        // for i in 0..8{
        //     leds[i].on();
        //     delay.delay_ms(500u32);
        //     leds[i].off();
        //     delay.delay_ms(500u32);
        // }
        
    }
}