// Coding conventions
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![deny(missing_docs)]

// Features
#![feature(conservative_impl_trait)]
#![feature(custom_derive)]
#![feature(proc_macro)]

//! Keep.me
//!
//! Documentation (TO DO)

// Crates import
extern crate crypto;
extern crate rustc_serialize;
extern crate futures;
#[macro_use] extern crate hyper;
extern crate hyper_openssl;
extern crate hyper_native_tls;
extern crate net2;
extern crate num_cpus;
extern crate tokio_core;
extern crate pretty_env_logger;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
#[cfg(all(test, feature = "unstable"))] extern crate test;
extern crate jwt;
extern crate unicase;

// root module reexports
pub use error::{Result, Error};

// Submodules
pub mod error;
pub mod core;
pub mod utils;
