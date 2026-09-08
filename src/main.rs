use crate::light::{Light, LightState};
use std::io::stdin;
use tracing::{error, trace};

mod light;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let light = Light::new();

    let mut light = LightState::Off(light);
    trace!("Light off: {}", light.is_off());
    trace!("Light on: {}", light.is_on());

    let mut input = String::new();

    loop {
        input.clear();
        stdin().read_line(&mut input).expect("Failed to read line");

        trace!("Input: {}", input.trim());

        match input.trim() {
            "on" => light = light.turn_on(),
            "off" => light = light.turn_off(),

            "exit" => {
                trace!("Exiting...");
                break;
            }
            _ => {
                error!("Invalid input. Please type 'on', 'off' or 'exit'.");
            }
        }

        trace!("Light is on: {}", light.is_on());
    }
}
