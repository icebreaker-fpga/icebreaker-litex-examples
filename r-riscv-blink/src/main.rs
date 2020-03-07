#![no_std]
#![no_main]

extern crate panic_halt;

use icebesoc_pac;
use fomu_rt::entry;

mod timer;
mod debug;

use timer::Timer;

const SYSTEM_CLOCK_FREQUENCY: u32 = 12_000_000;

// This is the entry point for the application.
// It is not allowed to return.
#[entry]
fn main() -> ! {
    let peripherals = icebesoc_pac::Peripherals::take().unwrap();

    let mut timer = Timer::new(peripherals.TIMER0);

    loop {
        println!("a");
        msleep(&mut timer, 80);
    }
}

fn msleep(timer: &mut Timer, ms: u32) {
    timer.disable();

    timer.reload(0);
    timer.load(SYSTEM_CLOCK_FREQUENCY / 10 * ms);

    timer.enable();

    // Wait until the time has elapsed
    while timer.value() > 0 {}
}
