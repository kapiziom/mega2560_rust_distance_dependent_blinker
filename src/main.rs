#![no_std]
#![no_main]

use panic_halt as _;

use arduino_hal::prelude::*;
use arduino_hal::{delay_us, Peripherals};
use arduino_hal::hal::port::mode::{Input, Output};
use arduino_hal::hal::port::{Pin};
use arduino_hal::port::mode::Floating;
use avr_device::atmega2560::{TC1, TC3};


#[arduino_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut led = pins.d13.into_output().downgrade();

    // Ultrasonic sensor pins
    let mut trigger = pins.d2.into_output().downgrade();
    let echo = pins.d3.into_floating_input().downgrade();

    // Timer for measurement distance
    let timer_distance = dp.TC1;

    // Timer helper led toggle
    let timer_led = dp.TC3;

    // Setting the prescaler to 64 to achieve time steps of 4 µs
    timer_distance.tccr1b.write(|w| w.cs1().prescale_64());
    timer_led.tccr3b.write(|w| w.cs3().prescale_1024());

    let mut blink_delay = 500; //ms

    let min_distance = 3; // cm
    let max_distance = 100; // cm

    loop {
        let distance_to_target = measure_distance(&timer_distance, &mut trigger, &echo);
        // ufmt::uwriteln!(&mut serial, "distance: {} cm", distance_to_target);

        if distance_to_target < min_distance || distance_to_target > max_distance {
            led.set_low();
            arduino_hal::delay_ms(1000);
            continue;
        }

        blink_delay = map_distance_to_delay(distance_to_target);

        let current_time = millis(&timer_led);

        if current_time >= blink_delay as u32 {
            led.toggle();

            // reset led timer
            timer_led.tcnt3.write(|w| unsafe { w.bits(0) });
        }

        arduino_hal::delay_ms(10);
    }
}

// read timer and get milliseconds
fn millis(timer: &TC3) -> u32 {
    let timer_value = timer.tcnt3.read().bits();
    let micros = (timer_value as u32) * 64;
    return micros / 1000
}

// map distance to delay milliseconds
fn map_distance_to_delay(distance: u16) -> u16 {
    distance * 15
}

fn measure_distance(
    timer: &TC1,
    trigger_pin: &mut Pin<Output>,
    echo_pin: &Pin<Input<Floating>>)
    -> u16
{
    // reset timer at the start of each measurement
    timer.tcnt1.write(|w| w.bits(0));

    // setting the trigger pin high for 10 us sends out the correct sound pulse
    trigger_pin.set_high();
    delay_us(10);
    trigger_pin.set_low();

    // the echo pins switches to high when/if our pulse bounces back, so we wait until that happens
    while echo_pin.is_low() {
        // Return the distance as 0 when no object is detected within the 50k timer value.
        if timer.tcnt1.read().bits() >= 50_000 {
            return 0u16;
        }
    }

    // we reset the timer here so we have a clean slate to calculate how long the echo pin is set to high
    timer.tcnt1.write(|w| w.bits(0));

    // waiting with timer running until echo pin returns to low
    while echo_pin.is_high() {}

    // reading distance as the value of the timer multiplied by 4, as each clock tick is 4 us
    let clock_ticks_as_us = timer.tcnt1.read().bits().saturating_mul(4);

    // If the echo pin remains high for too long and exceeds the bounds of u16 int, we consider it a bad reading and return 0.
    let distance = match clock_ticks_as_us {
        u16::MAX => 0,
        // calculate the distance from the timer value.
        _ => clock_ticks_as_us / 58,
    };

    distance
}