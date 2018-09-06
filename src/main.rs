#![feature(const_fn)]


extern crate dotenv;
use dotenv::dotenv;

extern crate hello_rocket_lib;
use hello_rocket_lib::rocket;



fn main() {
    dotenv().ok();
    rocket().launch();
}

