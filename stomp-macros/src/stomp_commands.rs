use syn;
use quote;

use attrs::{ Attributes, FieldAttributes };

pub fn expand(ast: &syn::MacroInput, attrs: &Attributes, field_attrs: &FieldAttributes) -> quote::Tokens {
    let name = &ast.ident;

    let context = attrs
        .get("context")
        .map(|s| syn::parse_type(s.into()).unwrap())
        .expect("#[derive(StompCommands)] must be used with #[stomp(context = \"SomeContext\")]");

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    quote! {
        impl #impl_generics ::stomp::StompCommands for #name #ty_generics #where_clause {
            fn commands() -> ::std::vec::Vec<::clap::App<'static, 'static>> {
                vec![]
            }
            fn parse(_matches: ::clap::ArgMatches) -> Self {
                unimplemented!()
            }
        }

        impl #impl_generics ::stomp::Executor for #name #ty_generics #where_clause {
            type Context = #context;
            fn run(self, _context: Self::Context) {
                unimplemented!()
            }
        }
    }
}
