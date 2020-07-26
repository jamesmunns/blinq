//! nrf52 blink example
//!
//! This is for the nrf52840-dk.
//!
//! LED1 will blink SOS in morse code
//! LED2 will blink HELLO. in morse code
#![no_std]
#![no_main]

use blinq::{consts, patterns, Blinq};
use cortex_m_rt::entry;
use embedded_hal::blocking::delay::DelayMs;
use nrf52840_hal::{
    self as hal,
    gpio::{
        p0::{Parts as P0Parts, P0_13, P0_14},
        Level, Output, PushPull,
    },
    Timer,
};
use panic_reset as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = hal::pac::Peripherals::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);
    let gpios = P0Parts::new(board.P0);

    let led1 = gpios.p0_13.into_push_pull_output(Level::High);
    let led2 = gpios.p0_14.into_push_pull_output(Level::High);

    let mut blinq_sos: Blinq<consts::U1, P0_13<Output<PushPull>>> = Blinq::new(led1, true);
    let mut blinq_hello: Blinq<consts::U6, P0_14<Output<PushPull>>> = Blinq::new(led2, true);

    loop {
        if blinq_sos.idle() && blinq_hello.idle() {
            rprintln!("Sleep...");
            blinq_sos.enqueue(patterns::morse::SOS);
            blinq_hello.enqueue(patterns::morse::H);
            blinq_hello.enqueue(patterns::morse::E);
            blinq_hello.enqueue(patterns::morse::L);
            blinq_hello.enqueue(patterns::morse::L);
            blinq_hello.enqueue(patterns::morse::O);
            blinq_hello.enqueue(patterns::morse::FULL_STOP);
            timer.delay_ms(1000u32);
        } else {
            rprintln!("Step...");
            blinq_sos.step();
            blinq_hello.step();
            timer.delay_ms(250u32);
        }
    }
}
