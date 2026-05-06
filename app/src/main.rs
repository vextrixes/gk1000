extern crate gk1000_controller;

fn main() {
    match gk1000_controller::RGBController::default().set_static_effect(0, 0, 255, 16, None) {
        Ok(_) => { println!("ok") }
        Err(err) => { println!("err: {err}") }
    }
}
