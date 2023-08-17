
pub mod ip;
pub mod rect;
pub mod coin;
pub mod poem;
pub mod args;
pub mod environ;
pub mod shuffle;

pub mod dave {
    pub fn hello() {
        println!("Hello, NyteOwl!");
    }
}

pub mod message {
    pub fn error(msg: &str) {
        eprintln!("{}", msg);   
    }
}

pub mod closures {
    // Return an adder closure
    pub fn make_adder(a: i32) -> impl Fn(i32) -> i32 {
        move |b| a + b
    }
}
