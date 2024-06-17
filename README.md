Distance-Dependent LED Blinker (Mega2560)
==============

This project provides a Rust-based program for the Arduino Mega2560 that controls the blinking of an LED based on the distance measured by an ultrasonic sensor. The blink rate of the LED changes depending on the measured distance from an object.

## Features

- **Distance Measurement**: Utilizes an ultrasonic sensor to measure the distance to an object.
- **LED Blinking**: Adjusts the blink rate of an LED based on the measured distance.
- **Rust Programming**: Demonstrates the use of Rust with Arduino Mega2560, including timer usage and GPIO control.

## Components Used

- **Mega2560**
- **Ultrasonic Sensor (HC-SR04)**
- **LED**
- **Rust**
- **`avr-hal`, `avr-device` crates**

## How It Works

1. **Setup**: The program configures the GPIO pins and timers.
2. **Trigger Pulse**: Sends a 10 µs pulse from the trigger pin to the ultrasonic sensor.
3. **Echo Measurement**: Waits for the echo pin to signal the return pulse, measuring the time taken.
4. **Distance Calculation**: Converts the measured time to distance in centimeters.
5. **LED Control**: Maps the distance to a delay value and toggles the LED accordingly. The closer an object is, the faster the LED blinks.

   The program calculates a delay value based on the measured distance:
   - If the distance is closer, the delay is shorter, resulting in faster LED blinking.
   - If the distance is farther, the delay is longer, resulting in slower LED blinking.
