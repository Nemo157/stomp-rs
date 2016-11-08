#![feature(proc_macro, proc_macro_lib)]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

mod from_lit;
mod attrs;
mod stomp_command;
mod stomp_commands;

#[proc_macro_derive(StompCommand)]
pub fn stomp_command(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut ast = syn::parse_macro_input(&input.to_string()).unwrap();
    let attrs = attrs::extract_attrs(&mut ast);
    let expanded = stomp_command::expand(&ast, &attrs);
    quote!(#ast #expanded).to_string().parse().unwrap()
}

#[proc_macro_derive(StompCommands)]
pub fn stomp_commands(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut ast = syn::parse_macro_input(&input.to_string()).unwrap();
    let attrs = attrs::extract_attrs(&mut ast);
    let expanded = stomp_commands::expand(&ast, &attrs);
    quote!(#ast #expanded).to_string().parse().unwrap()
}
