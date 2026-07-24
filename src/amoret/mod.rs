/* mods */

pub mod cli;
pub mod config;
mod rpc;
mod plugins;

pub use rpc::run;
