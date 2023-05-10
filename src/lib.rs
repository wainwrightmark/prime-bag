#![cfg_attr(not(any(test, feature = "std")), no_std)]
#[macro_use]
extern crate static_assertions;



pub mod prime_bag;
pub mod iter;
// mod nonzero_u8;
mod helpers;

