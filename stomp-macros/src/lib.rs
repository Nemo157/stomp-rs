#![feature(proc_macro, proc_macro_lib)]
#![feature(type_ascription)]

#[macro_use]
extern crate lazy_static;
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

mod attr;
mod attrs;
mod field;
mod stomp_command;
mod stomp_commands;

#[proc_macro_derive(StompCommand)]
pub fn stomp_command(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut ast = syn::parse_macro_input(&input.to_string()).unwrap();
    let (attrs, field_attrs) = attrs::extract_attrs(&mut ast);
    let expanded = stomp_command::expand(&ast, &attrs, &field_attrs);
    quote!(#ast #expanded).to_string().parse().unwrap()
}

#[proc_macro_derive(StompCommands)]
pub fn stomp_commands(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input(&input.to_string()).unwrap();
    let expanded = stomp_commands::expand(&ast);
    quote!(#ast #expanded).to_string().parse().unwrap()
}
