
#![no_std]
#![no_main]

// Macro for startup function
use cortex_m_rt::entry;
// Ensures halt on program panic
use panic_halt as _;

// Aliasing hal crate
use rp2040_hal as hal;

// Getting access to PAC
use hal::pac;


use embedded_hal::digital::v2::OutputPin;
use embedded_time::{fixed_point::FixedPoint, duration::Extensions};
use rp2040_hal::clocks::Clock;



// Linking boot block at the start of our program image
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

const XTAL_FREQ_HZ: u32 = 12_000_000u32;

// Specifying entry point for the controller

#[entry]
fn main() -> ! {
    // Grab singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // setup the watchdog driver
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    ).ok()
    .unwrap();
    
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    // SIO -> Single-cycle I/O
    let sio = hal::Sio::new(pac.SIO);

    // set pins to default
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0, 
        pac.PADS_BANK0, 
        sio.gpio_bank0, 
        &mut pac.RESETS
    );

    //  // // Everything above is configuration

    let exec = move || {
        // setup code
        // let mut led_pin = pins.gpio25.into_push_pull_output();


        loop {
            // Execution Loop (can be empty)
        }

    };

    exec()
}

