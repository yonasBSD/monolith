#[macro_use]
extern crate clap;

#[macro_use]
mod macros;

pub mod css;
pub mod html;
pub mod js;
pub mod opts;
pub mod url;
pub mod utils;

#[cfg(test)]
pub mod tests;
