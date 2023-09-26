#![no_std]
#![no_main]

// if release mode, use panic_halt else panic_semihosting
#[cfg(debug_assertions)]
use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

#[cfg(not(debug_assertions))]
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

// Macro for hprintln which returns void in release mode and prints in debug mode
macro_rules! hprintln {
    ($($tt:tt)*) => {
        #[cfg(debug_assertions)]
        {
            cortex_m_semihosting::hprintln!($($tt)*);
            ()
        }
        #[cfg(not(debug_assertions))]
        {
            ()
        }
    }
}

use cortex_m_rt::entry;
use stm32l0xx_hal::{
    pac,
    pwr::PWR,
    prelude::*, 
    rcc::Config, 
    rtc::Rtc, 
};

use chrono::prelude::*;

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!");

    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    // Configure the clock.
    let mut rcc = dp.RCC.freeze(Config::hsi16());

    // Acquire the GPIOA peripheral. This also enables the clock for GPIOB in
    // the RCC register.
    let gpioa = dp.GPIOA.split(&mut rcc);

    // Configure PA9 as output.
    let mut led = gpioa.pa9.into_push_pull_output();

    // Configure RTC
    let pwr = PWR::new(dp.PWR, &mut rcc);

    let mut rtc = Rtc::new(dp.RTC, &mut rcc, &pwr, None).unwrap();

    // Get the delay provider.
    let mut delay = cp.SYST.delay(rcc.clocks);

    loop {
        let now = rtc.now();
        let datetime: DateTime<Utc> = DateTime::from_timestamp(now.timestamp(), now.timestamp_subsec_nanos()).unwrap();

        hprintln!("{}-{}-{} {}:{}:{}", datetime.year(), datetime.month(), datetime.day(), datetime.hour(), datetime.minute(), datetime.second());

        // The led turns on for 500ms and then off for 500ms.
        led.set_high().unwrap();
        delay.delay_ms(500_u16);
        
        led.set_low().unwrap();
        delay.delay_ms(500_u16);
    }
}
