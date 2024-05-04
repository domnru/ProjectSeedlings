#![no_std]
#![no_main]

use core::{alloc, panic::PanicInfo};

use cortex_m::asm::nop;
use embedded_hal::digital::OutputPin;
use rp_pico as bsp;

use bsp::{
    entry,
    hal::pac,
    hal,
};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // !!Write panic for specific needs!!
    loop {}
}

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let sio = hal::Sio::new(pac.SIO);
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    let mut led_pin = pins.led.into_push_pull_output();

    loop {
        led_pin.set_high().unwrap();
        primitive_delay_ms(500);

        led_pin.set_low().unwrap();
        primitive_delay_ms(500);
    }
}

fn primitive_delay_ms(iter: u32) {
    for _ in 1..(iter/1000) {
        nop();
    }
}
