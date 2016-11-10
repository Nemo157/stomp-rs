use syn;
use quote;

pub fn expand_commands(cmds: &[(&syn::Ident, &syn::Ty)]) -> quote::Tokens {
    let types = cmds.iter().map(|&(_ident, ty)| ty);
    quote! { vec![ #(<#types as ::stomp::StompCommand>::command()),* ] }
}

fn expand_parse(me: &syn::Ident, cmds: &[(&syn::Ident, &syn::Ty)], name: &syn::Ident, matches: &syn::Ident) -> quote::Tokens {
    let variants = cmds.iter().map(|&(ident, ty)| {
        let name = ident.as_ref().to_lowercase();
        quote! { #name => #me::#ident(<#ty as ::stomp::StompCommand>::parse(#matches)) }
    });
    quote! {
        match #name {
            #(#variants,)*
            _ => unreachable!(),
        }
    }
}

pub fn expand(ast: &syn::MacroInput) -> quote::Tokens {
    let ident = &ast.ident;
    let name = "name".into(): syn::Ident;
    let matches = "matches".into(): syn::Ident;

    let cmds: Vec<_> = match ast.body {
        syn::Body::Enum(ref variants) => {
            variants.iter()
                .map(|variant| match variant.data {
                    syn::VariantData::Tuple(ref fields) => {
                        if fields.len() == 1 {
                            (&variant.ident, &fields[0].ty)
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

    let commands = expand_commands(&cmds);
    let parse = expand_parse(ident, &cmds, &name, &matches);
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    quote! {
        impl #impl_generics ::stomp::StompCommands for #ident #ty_generics #where_clause {
            fn commands() -> ::std::vec::Vec<::clap::App<'static, 'static>> {
                #commands
            }
            fn parse(#name: &str, #matches: &::clap::ArgMatches) -> Self {
                #parse
            }
        }
    }
}
