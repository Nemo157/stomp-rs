use syn;
use quote;

use attrs::{ Attributes, FieldAttributes };
use field::{ Arg, Field, Subcommand };

fn expand_arg(arg: &Arg) -> quote::Tokens {
    let name = arg.name;
    let ty = arg.ty;
    let short = arg.short.as_ref().map(|s| quote! { .short(#s) });
    let long = arg.long.map(|s| quote! { .long(#s) });
    let value_name = arg.value_name.map(|s| quote! { .value_name(#s) });
    let takes_value = arg.takes_value;
    let index = arg.index.map(|i| quote! { .index(#i) });
    let docs = (arg.summary.to_string() + "\n\n" + arg.docs).trim().to_string();
    let multiple = arg.multiple;
    let default_value = arg.default_value.map(|d| quote! { .default_value(#d) });
    let min_values = arg.min_values.map(|m| quote! { .min_values(#m) });
    let max_values = arg.max_values.map(|m| quote! { .max_values(#m) });
    let required = arg.required;
    let validator = if arg.takes_value {
        Some(quote! {
            .validator(|s| {
                <#ty as ::std::str::FromStr>::from_str(&s)
                    .map(|_| ())
                    .map_err(|e| format!("failed to parse value {:?} for argument '{}': {}", s, #name, e))
            })
        })
    } else {
        None
    };

    quote! {
        ::clap::Arg::with_name(#name)
            #short
            #long
            #value_name
            #index
            .help(#docs)
            .takes_value(#takes_value)
            .multiple(#multiple)
            #default_value
            #min_values
            #max_values
            .required(#required)
            #validator
    }
}

fn expand_args<'a, 'b: 'a, I>(args: I) -> quote::Tokens where I: Iterator<Item=&'a Arg<'b>> {
    let args = args.map(expand_arg);
    quote! { .args(&[#(#args),*]) }
}

fn expand_subcommand(subcommand: &Subcommand) -> quote::Tokens {
    let ty = subcommand.ty;
    let required = if subcommand.is_optional {
        None
    } else {
        Some(quote! { .setting(::clap::AppSettings::SubcommandRequiredElseHelp) })
    };

    quote! {
        .subcommands(<#ty as ::stomp::StompCommands>::commands())
        #required
    }
}

fn expand_command(ast: &syn::MacroInput, attrs: &Attributes, fields: &[Field]) -> quote::Tokens {
    let name = attrs.get("name").map(|a| a.into())
            .unwrap_or_else(|| syn::Lit::from(ast.ident.as_ref().to_lowercase()));

    let version = attrs.get("version").map(|v| quote! { .version(#v) });

    let author = attrs.get("author").map(|a| quote! { .author(#a) });

    let args = expand_args(fields.iter().filter_map(|field| field.arg()));
    let subcommand = fields.iter()
        .filter_map(|field| field.subcommand())
        .find(|_| true)
        .map(expand_subcommand);

    let ref summary = attrs.summary;
    let ref docs = attrs.docs;
    let alias = attrs.get("alias").map(|a| quote! { .alias(#a) });

    quote! {
        ::clap::App::new(#name)
            #version
            #author
            #args
            #subcommand
            .about(#summary)
            .after_help(#docs)
            #alias
    }
}

fn expand_parse_arg(arg: &Arg, matches: &syn::Ident) -> quote::Tokens {
    let ident = arg.ident;
    let name = arg.name;
    let value = if arg.is_counter {
        quote! { #matches.occurrences_of(#name) }
    } else {
        if arg.takes_value {
            if arg.multiple {
                quote! {
                    #matches
                        .values_of(#name)
                        .map(|vs| vs.map(|v| v.parse().unwrap()).collect())
                        .unwrap_or_else(|| Vec::new())
                }
            } else {
                if arg.is_optional {
                    quote! {
                        #matches
                            .value_of(#name)
                            .map(|a| a.parse().unwrap())
                    }
                } else {
                    quote! {
                        #matches
                            .value_of(#name).unwrap()
                            .parse().unwrap()
                    }
                }
            }
        } else {
            quote! { #matches.is_present(#name) }
        }
    };

    quote! {
        #ident: #value
    }
}

fn expand_parse_subcommand(cmd: &Subcommand, matches: &syn::Ident) -> quote::Tokens {
    let ident = cmd.ident;
    let ty = cmd.ty;

    let (default, wrapper);
    if cmd.is_optional {
        default = quote! { None };
        wrapper = Some(quote! { Some });
    } else {
        default = quote! { unreachable!() };
        wrapper = None;
    }

    quote! {
        #ident: match #matches.subcommand() {
            (name, Some(matches)) => #wrapper(<#ty as ::stomp::StompCommands>::parse(name, matches)),
            (_, None) => #default,
        }
    }
}

fn expand_parse_field(field: &Field, matches: &syn::Ident) -> quote::Tokens {
    match *field {
        Field::Arg(ref arg) => expand_parse_arg(arg, matches),
        Field::Subcommand(ref cmd) => expand_parse_subcommand(cmd, matches),
    }
}

fn expand_parse(ast: &syn::MacroInput, fields: &[Field], matches: &syn::Ident) -> quote::Tokens {
    let name = &ast.ident;
    let fields = fields.iter().map(|field| expand_parse_field(field, matches));
    quote! {
        #name {
            #( #fields ),*
        }
    }
}

pub fn expand(ast: &syn::MacroInput, attrs: &Attributes, field_attrs: &FieldAttributes) -> quote::Tokens {
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

    let ident = &ast.ident;
    let command = expand_command(ast, attrs, &fields);
    let matches = "matches".into(): syn::Ident;
    let parse = expand_parse(ast, &fields, &matches);
    let allow_unused = syn::Attribute {
        style: syn::AttrStyle::Outer,
        value: syn::MetaItem::List(syn::Ident::from("allow"), vec![
            syn::NestedMetaItem::MetaItem(
                syn::MetaItem::Word(syn::Ident::from("unused_variables"))
            ),
        ]),
        is_sugared_doc: false,
    };
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    quote! {
        impl #impl_generics ::stomp::StompCommand for #ident #ty_generics #where_clause {
            fn command() -> ::clap::App<'static, 'static> {
                #command
            }
            #allow_unused
            fn parse(#matches: &::clap::ArgMatches) -> Self {
                #parse
            }
        }
    }
}
