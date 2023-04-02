#![allow(dead_code)]
#![allow(unused_assignments)]
#![feature(array_zip)]

mod checkpoints;
mod math;
mod rendering;

fn main() {
    checkpoints::shaded_circle::run().unwrap();
}
