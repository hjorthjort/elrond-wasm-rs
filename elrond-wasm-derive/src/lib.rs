

#![allow(dead_code)]
#![allow(stable_features)]

// ensure we don't run out of macro stack
#![recursion_limit="1024"]

extern crate proc_macro;

#[macro_use]
extern crate syn;

#[macro_use]
extern crate quote;

mod arg_def;
mod arg_extract;
mod arg_regular;
mod arg_str_serialize;
mod contract_gen;
mod contract_gen_callback;
mod contract_gen_event;
mod contract_gen_finish;
mod contract_gen_method;
mod contract_gen_module;
mod contract_gen_payable;
mod contract_gen_storage;
mod contract_impl;
mod contract_macro_main;
mod contract_macro_module;
mod parse_attr;
mod snippets;
mod util;
mod reserved;

mod callable;
mod callable_gen;

#[proc_macro_attribute]
pub fn contract(
  args: proc_macro::TokenStream,
  input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {

  contract_macro_main::process_contract(args, input)

}

#[proc_macro_attribute]
pub fn module(
  args: proc_macro::TokenStream,
  input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {

  contract_macro_module::process_module(args, input)

}

#[proc_macro_attribute]
pub fn callable(
  args: proc_macro::TokenStream,
  input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {

  callable::process_callable(args, input)

}
