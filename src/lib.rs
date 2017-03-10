#![feature(plugin)]
#![plugin(dotenv_macros)]
extern crate hyper;
extern crate hyper_native_tls;

pub mod status;
pub mod service;
