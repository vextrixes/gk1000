mod color;
mod commands;
mod completions;
mod error_handler;

use clap::{CommandFactory, Parser};
use commands::{ArgsCli, Cmds};
use completions::print_completions;
use error_handler::error_handler;

fn main() {
    let args = ArgsCli::parse();
    match args.subcommand {
        Cmds::Static(effect) => {
            match gk1000_controller::RGBController::default().set_static_effect(
                effect.color.red,
                effect.color.green,
                effect.color.blue,
                effect.brightness,
                Some(effect.full_color),
            ) {
                Ok(_) => println!("Changed effect to Static"),
                Err(err) => error_handler(err),
            };
        }
        Cmds::SingleOn(effect) => {
            match gk1000_controller::RGBController::default().set_single_on_effect(
                effect.color.red,
                effect.color.green,
                effect.color.blue,
                effect.brightness,
                effect.speed,
                Some(effect.full_color),
            ) {
                Ok(_) => println!("Changed effect to Single On"),
                Err(err) => error_handler(err),
            };
        }
        Cmds::SingleOff(effect) => {
            match gk1000_controller::RGBController::default().set_single_off_effect(
                effect.color.red,
                effect.color.green,
                effect.color.blue,
                effect.brightness,
                effect.speed,
                Some(effect.full_color),
            ) {
                Ok(_) => println!("Changed effect to Single Off"),
                Err(err) => error_handler(err),
            };
        }
        Cmds::Glittering(effect) => {
            match gk1000_controller::RGBController::default().set_glittering_effect(
                effect.color.red,
                effect.color.green,
                effect.color.blue,
                effect.brightness,
                effect.speed,
                Some(effect.full_color),
            ) {
                Ok(_) => println!("Changed effect to Glittering"),
                Err(err) => error_handler(err),
            };
        }
        Cmds::Rain(effect) => {
            match gk1000_controller::RGBController::default().set_rain_effect(
                effect.color.red,
                effect.color.green,
                effect.color.blue,
                effect.brightness,
                effect.speed,
                Some(effect.full_color),
            ) {
                Ok(_) => println!("Changed effect to Rain"),
                Err(err) => error_handler(err),
            };
        }
        Cmds::Colorful(effect) => {
            match gk1000_controller::RGBController::default().set_colorful_effect(
                effect.color.red,
                effect.color.green,
                effect.color.blue,
                effect.brightness,
                effect.speed,
                Some(effect.full_color),
            ) {
                Ok(_) => println!("Changed effect to Colorful"),
                Err(err) => error_handler(err),
            };
        }
        Cmds::Breath(effect) => {
            match gk1000_controller::RGBController::default().set_breath_effect(
                effect.color.red,
                effect.color.green,
                effect.color.blue,
                effect.brightness,
                effect.speed,
                Some(effect.full_color),
            ) {
                Ok(_) => println!("Changed effect to Breath"),
                Err(err) => error_handler(err),
            };
        }
        Cmds::Spectrum(effect) => {
            match gk1000_controller::RGBController::default()
                .set_spectrum_effect(effect.brightness, effect.speed)
            {
                Ok(_) => println!("Changed effect to Spectrum"),
                Err(err) => error_handler(err),
            };
        }
        Cmds::CentrifugalWave(effect) => {
            match gk1000_controller::RGBController::default().set_centrifugal_wave_effect(
                effect.color.red,
                effect.color.green,
                effect.color.blue,
                effect.brightness,
                effect.speed,
                Some(effect.full_color),
            ) {
                Ok(_) => println!("Changed effect to Centrifugal wave"),
                Err(err) => error_handler(err),
            };
        }
        Cmds::VerticalWave(effect) => {
            match gk1000_controller::RGBController::default().set_vertical_wave_effect(
                effect.color.red,
                effect.color.green,
                effect.color.blue,
                effect.brightness,
                effect.speed,
                effect.direction.to_u8(),
                Some(effect.full_color),
            ) {
                Ok(_) => println!("Changed effect to Vertical wave"),
                Err(err) => error_handler(err),
            };
        }
        Cmds::HorizontalWave(effect) => {
            match gk1000_controller::RGBController::default().set_horizontal_wave_effect(
                effect.color.red,
                effect.color.green,
                effect.color.blue,
                effect.brightness,
                effect.speed,
                effect.direction.to_u8(),
                Some(effect.full_color),
            ) {
                Ok(_) => println!("Changed effect to Horizontal wave"),
                Err(err) => error_handler(err),
            };
        }
        Cmds::Rotating(effect) => {
            match gk1000_controller::RGBController::default().set_rotating_effect(
                effect.color.red,
                effect.color.green,
                effect.color.blue,
                effect.brightness,
                effect.speed,
                Some(effect.full_color),
            ) {
                Ok(_) => println!("Changed effect to Rotating"),
                Err(err) => error_handler(err),
            };
        }
        Cmds::Explosion(effect) => {
            match gk1000_controller::RGBController::default().set_explosion_effect(
                effect.color.red,
                effect.color.green,
                effect.color.blue,
                effect.brightness,
                effect.speed,
                Some(effect.full_color),
            ) {
                Ok(_) => println!("Changed effect to Explosion"),
                Err(err) => error_handler(err),
            };
        }
        Cmds::Launch(effect) => {
            match gk1000_controller::RGBController::default().set_launch_effect(
                effect.color.red,
                effect.color.green,
                effect.color.blue,
                effect.brightness,
                effect.speed,
                Some(effect.full_color),
            ) {
                Ok(_) => println!("Changed effect to Launch"),
                Err(err) => error_handler(err),
            };
        }
        Cmds::Ripples(effect) => {
            match gk1000_controller::RGBController::default().set_ripples_effect(
                effect.color.red,
                effect.color.green,
                effect.color.blue,
                effect.brightness,
                effect.speed,
                Some(effect.full_color),
            ) {
                Ok(_) => println!("Changed effect to Ripples"),
                Err(err) => error_handler(err),
            };
        }
        Cmds::Snake(effect) => {
            match gk1000_controller::RGBController::default().set_snake_effect(
                effect.color.red,
                effect.color.green,
                effect.color.blue,
                effect.brightness,
                effect.speed,
                Some(effect.full_color),
            ) {
                Ok(_) => println!("Changed effect to Snake"),
                Err(err) => error_handler(err),
            };
        }
        Cmds::Pulse(effect) => {
            match gk1000_controller::RGBController::default().set_pulse_effect(
                effect.color.red,
                effect.color.green,
                effect.color.blue,
                effect.brightness,
                effect.speed,
                Some(effect.full_color),
            ) {
                Ok(_) => println!("Changed effect to Pulse"),
                Err(err) => error_handler(err),
            };
        }
        Cmds::Tilt(effect) => {
            match gk1000_controller::RGBController::default().set_tilt_effect(
                effect.color.red,
                effect.color.green,
                effect.color.blue,
                effect.brightness,
                effect.speed,
                Some(effect.full_color),
            ) {
                Ok(_) => println!("Changed effect to Tilt"),
                Err(err) => error_handler(err),
            };
        }
        Cmds::Shuttle(effect) => {
            match gk1000_controller::RGBController::default().set_shuttle_effect(
                effect.color.red,
                effect.color.green,
                effect.color.blue,
                effect.brightness,
                effect.speed,
                Some(effect.full_color),
            ) {
                Ok(_) => println!("Changed effect to Shuttle"),
                Err(err) => error_handler(err),
            };
        } //TODO: Static per key effect support
        Cmds::Completions(args) => print_completions(args.shell, &mut ArgsCli::command()),
    }
}
