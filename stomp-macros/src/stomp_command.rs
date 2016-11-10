use syn;
use quote;

use attrs::{ Attributes, FieldAttributes };
use field::{ Arg, Field, Subcommand };

fn expand_arg(arg: &Arg) -> quote::Tokens {
    let name = arg.name;
    let short = arg.short.as_ref().map(|s| quote! { .short(#s) });
    let long = arg.long.map(|s| quote! { .long(#s) });
    let value_name = arg.value_name.map(|s| quote! { .value_name(#s) });
    let takes_value = arg.takes_value;
    let index = arg.index.map(|i| quote! { .index(#i) });
    let ref docs = arg.docs;
    let multiple = arg.multiple;

    quote! {
        ::clap::Arg::with_name(#name)
            #short
            #long
            #value_name
            #index
            .help(#docs)
            .takes_value(#takes_value)
            .multiple(#multiple)
    }
}

fn expand_args<'a, 'b: 'a, I>(args: I) -> quote::Tokens where I: Iterator<Item=&'a Arg<'b>> {
    let args = args.map(expand_arg);
    quote! { .args(&[#(#args),*]) }
}

fn expand_subcommand(subcommand: &Subcommand) -> quote::Tokens {
    let ty = subcommand.ty;
    let required = if subcommand.required {
        Some(quote! { .setting(::clap::AppSettings::SubcommandRequiredElseHelp) })
    } else {
        None
    };

    quote! {
        .subcommands(<#ty as ::stomp::StompCommands>::commands())
        #required
    }
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

    let args = expand_args(fields.iter().filter_map(|field| field.arg()));
    let subcommand = fields.iter()
        .filter_map(|field| field.subcommand())
        .find(|_| true)
        .map(expand_subcommand);

    let ref docs = attrs.docs;

    quote! {
        ::clap::App::new(#name)
            #version
            #author
            #args
            #subcommand
            .about(#docs)
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
