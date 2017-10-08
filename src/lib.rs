#![feature(plugin)]
#![plugin(docopt_macros)]

#[macro_use]
extern crate serde_derive;

pub mod file;
pub mod json;
pub mod schema;
pub mod template;
pub mod parser;
pub mod option;
pub mod output;
