extern crate gk1000_controller;

fn main() {
    gk1000_controller::RGBController::default().set_color(0x9f, 0xff, 0xff);
    //gk1000_controller::RGBController::new().set_color(0, 225, 20);
}
