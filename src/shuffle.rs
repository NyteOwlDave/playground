#![allow(dead_code)]

use rand::prelude::*;
// use rand::Rng;

pub fn test1() {
    let mut v: Vec<u32> = (0..20).collect();
    let mut rng = rand::thread_rng();
    v.shuffle(&mut rng);
    println!("{:?}", v);
}
