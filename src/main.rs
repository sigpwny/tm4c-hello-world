#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;
// use embedded_hal::digital::OutputPin;
use embedded_hal::digital::v2::OutputPin;
use tm4c123x_hal as hal;
use tm4c123x_hal::prelude::SysctlExt;
use tm4c123x_hal::gpio::{gpiof::*, GpioExt, Input, Output, PullUp, PushPull};

#[entry]
fn main() -> ! {
    // asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    // Initialize peripherals
    let p = hal::Peripherals::take().unwrap();
    let mut sysctl = p.SYSCTL.constrain();

    // GPIOF for LED pins
    let mut gpiof = p.GPIO_PORTF.split(&sysctl.power_control);
    let mut led_red = gpiof.pf1.into_push_pull_output();
    let mut led_blue = gpiof.pf2.into_push_pull_output();
    let mut led_green = gpiof.pf3.into_push_pull_output();


    loop {
        // Turn the LED on
        led_red.set_high().unwrap();
        led_blue.set_high().unwrap();
        led_green.set_high().unwrap();

        // Delay
        asm::delay(8_000_000);

        // Turn the LED off
        led_red.set_low().unwrap();
        led_blue.set_low().unwrap();
        led_green.set_low().unwrap();

        // Delay
        asm::delay(8_000_000);
    }
}
