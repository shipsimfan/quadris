mod controller;
mod model;
mod view;

fn main() {
    colosseum::App::<controller::Game>::new()
}
