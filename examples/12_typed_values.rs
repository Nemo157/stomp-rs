#![feature(attr_literals)]
#![feature(custom_derive)]
#![feature(proc_macro)]

extern crate clap;
extern crate stomp;
#[macro_use]
extern crate stomp_macros;

use stomp::ParseApp;

// Create two arguments, a required positional which accepts multiple values
// and an optional '-l value'
#[derive(StompCommand)]
struct MyApp {
    /// A sequence of whole positive numbers, i.e. 20 25 30
    #[stomp(arg, min_values = 1)]
    seq: Vec<u32>,
    /// A length to use
    #[stomp(short = 'l', default_value = "10")]
    len: u32,
}

fn main() {
    let app = MyApp::parse();

    // Here we get a value of type u32 from our optional -l argument.
    let len = app.len;

    println!("len ({}) + 2 = {}", len, len + 2);

    // This code loops through all the values provided to "seq" and adds 2
    for v in app.seq {
        println!("Sequence part {} + 2: {}", v, v + 2);
    }
}
