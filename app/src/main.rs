extern crate gk1000_controller;

fn main() {
    match gk1000_controller::RGBController::default().set_color(0, 0, 255) {
        Ok(_) => { println!("ok") }
        Err(err) => { println!("err: {err}") }
    }
}
