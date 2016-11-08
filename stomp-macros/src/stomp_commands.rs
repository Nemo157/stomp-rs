use syn;
use quote;

use attrs::Attributes;

pub fn expand(ast: &syn::MacroInput, attrs: &Attributes) -> quote::Tokens {
    let name = &ast.ident;
    let context = attrs.root
        .get("context")
        .map(|s: String| syn::parse_path(&s).unwrap())
        .expect("#[derive(StompCommands)] must be used with #[stomp(context = \"SomeContext\")]");
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    quote! {
        impl #impl_generics ::stomp::StompCommands for #name #ty_generics #where_clause {
            fn commands() -> ::std::vec::Vec<::clap::App<'static, 'static>> {
                unimplemented!()
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
