#![allow(dead_code)]
#![feature(array_zip)]
#![allow(unused_assignments)]

mod checkpoints;
mod math;
mod rendering;

fn main() {
    checkpoints::shaded_circle::run().unwrap();
}
