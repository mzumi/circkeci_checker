#![feature(plugin)]
#![plugin(dotenv_macros)]
#[macro_use] extern crate hyper;

pub mod status;
pub mod service;
