#![feature(custom_derive, plugin, custom_attribute)]
#![plugin(yaqb_codegen)]

extern crate quickcheck;
#[macro_use] extern crate yaqb;

mod associations;
mod expressions;
mod filter;
mod filter_operators;
mod find;
mod insert;
mod internal_details;
mod joins;
mod macros;
mod order;
mod perf_details;
mod schema;
mod select;
mod transactions;
mod types;
mod types_roundtrip;
mod update;