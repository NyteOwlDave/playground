
#![allow(dead_code)]

extern crate playground;

// use playground::dave;
// use playground::ip;
// use playground::rect;
// use playground::coin;
// use playground::coin::Coin;

// use std::process;
// use std::env;
// use std::fs::File;
// use std::io::prelude::*;
// use std::error::Error;
// use playground::args;
// use playground::poem;
// use playground::message;
use playground::closures;

fn test1() {
    // args::test();
    // poem::poem();
    let result = closures::make_adder(1)(2);
    println!("{}", result);
}


fn test2() {
    let mut a = 2;
    let mut b = 3;
    a;b = 2;
    println!("{a} {b}");
}

fn main() {
    //use playground::shuffle;
    //shuffle::test1();
    test2();
}
