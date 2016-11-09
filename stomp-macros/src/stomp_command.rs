use syn;
use quote;

use attrs::{ self, Attributes, FieldAttributes };
use field::Field;

fn expand_arg(field: &Field) -> Option<quote::Tokens> {
    if field.is_subcommand {
        return None;
    }

    let name = field.name;

    let short = field.short.as_ref().map(|s| quote! { .short(#s) });

    let long = field.long.map(|s| quote! { .long(#s) });

    let takes_value = if field.takes_value {
        Some(quote! { .takes_value(true) })
    } else {
        None
    };

    Some(quote! {
        ::clap::Arg::with_name(#name)
            #short
            #long
            #takes_value
    })
}

fn expand_args(fields: &[Field]) -> quote::Tokens {
    let args = fields.iter().filter_map(expand_arg);
    quote! { .args(&[#(#args),*]) }
}

fn expand_command(ast: &syn::MacroInput, attrs: &Attributes, field_attrs: &FieldAttributes) -> quote::Tokens {
    let name = attrs.get("name").map(|a| a.into())
            .unwrap_or_else(|| syn::Lit::from(ast.ident.as_ref().to_lowercase()));

    let version = attrs.get("version").map(|v| quote! { .version(#v) });

    let author = attrs.get("author").map(|a| quote! { .author(#a) });

    let fields = match ast.body {
        syn::Body::Struct(syn::VariantData::Unit) => {
            Vec::new()
        }
        syn::Body::Struct(syn::VariantData::Struct(ref fields)) => {
            fields.iter()
                .map(|field| Field::from((field, field_attrs.get(field))))
                .collect()
        }
        syn::Body::Struct(syn::VariantData::Tuple(_)) => {
            panic!("#[derive(StompCommand)] is not supported on tuple structs")
        }
        syn::Body::Enum(_) => {
            panic!("#[derive(StompCommand)] is not supported on enums")
        }
    };

    let args = expand_args(&fields);

    quote! {
        ::clap::App::new(#name)
            #version
            #author
            #args
    }
}

pub fn expand(ast: &syn::MacroInput, attrs: &Attributes, field_attrs: &FieldAttributes) -> quote::Tokens {
    let name = &ast.ident;
    let command = expand_command(ast, attrs, field_attrs);
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    quote! {
        impl #impl_generics ::stomp::StompCommand for #name #ty_generics #where_clause {
            fn command() -> ::clap::App<'static, 'static> {
                #command
            }
            fn parse(_matches: ::clap::ArgMatches) -> Self {
                unimplemented!()
            }
        }
    }
}
