#![allow(dead_code)]
#![feature(array_zip)]

mod checkpoints;
mod math;
mod rendering;

fn main() {
    checkpoints::clock::run().unwrap();
}
