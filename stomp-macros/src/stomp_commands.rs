use syn;
use quote;

use attrs::Attributes;

pub fn expand_commands(ast: &syn::MacroInput) -> quote::Tokens {
    let commands: Vec<_> = match ast.body {
        syn::Body::Enum(ref variants) => {
            variants.iter()
                .map(|variant| match variant.data {
                    syn::VariantData::Tuple(ref fields) => {
                        if fields.len() == 1 {
                            &fields[0].ty
                        } else {
                            panic!("#[derive(StompCommands)] does not support enum variants with multiple fields")
                        }
                    }
                    syn::VariantData::Struct(_) => {
                        panic!("#[derive(StompCommands)] does not support struct enum variants")
                    }
                    syn::VariantData::Unit => {
                        panic!("#[derive(StompCommands)] does not support unit enum variants")
                    }
                })
                .collect()
        }
        syn::Body::Struct(_) => {
            panic!("#[derive(StompCommands)] is not supported on structs")
        }
    };

    quote! { vec![ #(<#commands as ::stomp::StompCommand>::command()),* ] }
}

pub fn expand(ast: &syn::MacroInput, attrs: &Attributes) -> quote::Tokens {
    let name = &ast.ident;

    let context = attrs
        .get("context")
        .map(|s| syn::parse_type(s.into()).unwrap())
        .expect("#[derive(StompCommands)] must be used with #[stomp(context = \"SomeContext\")]");

    let commands = expand_commands(ast);
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    quote! {
        impl #impl_generics ::stomp::StompCommands for #name #ty_generics #where_clause {
            fn commands() -> ::std::vec::Vec<::clap::App<'static, 'static>> {
                #commands
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
