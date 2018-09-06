//extern crate rocket;
//extern crate rocket_contrib;
//extern crate hello_rocket_lib;
//use hello_rocket_lib;
//use rocket;

#![feature(const_fn)]

#[macro_use] extern crate diesel_migrations;
embed_migrations!("migrations");
#[cfg(test)] mod user;