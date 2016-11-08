use syn;
use quote;

use attrs::Attributes;

pub fn expand(ast: &syn::MacroInput, attrs: &Attributes) -> quote::Tokens {
    let name = &ast.ident;
    let name_str = syn::Lit::Str(name.as_ref().to_lowercase(), syn::StrStyle::Cooked);
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    quote! {
        impl #impl_generics ::stomp::StompCommand for #name #ty_generics #where_clause {
            fn command() -> ::clap::App<'static, 'static> {
                ::clap::App::new(#name_str)
            }
            fn parse(_matches: ::clap::ArgMatches) -> Self {
                unimplemented!()
            }
        }
    }
}
