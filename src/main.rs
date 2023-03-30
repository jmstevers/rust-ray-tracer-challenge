#![allow(dead_code)]
#![feature(array_zip)]

mod checkpoints;
mod math;
mod rendering;

fn main() {
    checkpoints::shaded_circle::run().unwrap();
}
