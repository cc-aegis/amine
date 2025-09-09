#![allow(unused)]
#![deny(unsafe_op_in_unsafe_fn)]
#![feature(f16)]

pub mod cpu;
pub mod device;

// TODO: allow adding debug handler so debug output can be forwarded into tests easily without needing to rely on io