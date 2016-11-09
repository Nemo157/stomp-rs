use syn;

use attrs::Attributes;

pub enum Field<'a> {
    Arg(Arg<'a>),
    Subcommand(Subcommand<'a>),
}

pub struct Arg<'a> {
    pub name: &'a str,
    pub short: Option<String>,
    pub long: Option<&'a str>,
    pub index: Option<u64>,
    pub takes_value: bool,
}

pub struct Subcommand<'a> {
    pub ty: &'a syn::Ty,
    pub required: bool,
}

impl<'a> Field<'a> {
    pub fn arg(&self) -> Option<&Arg> {
        if let Field::Arg(ref arg) = *self {
            Some(arg)
        } else {
            None
        }
    }

    pub fn subcommand(&self) -> Option<&Subcommand> {
        if let Field::Subcommand(ref subcommand) = *self {
            Some(subcommand)
        } else {
            None
        }
    }
}

impl<'a> From<(&'a syn::Field, &'a Attributes)> for Field<'a> {
    fn from((field, attrs): (&'a syn::Field, &'a Attributes)) -> Field<'a> {
        if attrs.get_bool("subcommand") {
            Field::Subcommand(Subcommand::from((field, attrs)))
        } else {
            Field::Arg(Arg::from((field, attrs)))
        }
    }
}

impl<'a> From<(&'a syn::Field, &'a Attributes)> for Arg<'a> {
    fn from((field, attrs): (&'a syn::Field, &'a Attributes)) -> Arg<'a> {
        let name = attrs.get("name").map(|a| a.into())
                .unwrap_or_else(|| field.ident.as_ref().unwrap().as_ref());

        let index = attrs.get("index").map(|a| a.into(): u64);

        // Unlike clap we default to a flag option unless there's a attribute given
        // telling us to not do so
        let is_flag = !index.is_some();

        let long = attrs.get("long").map(|a| a.into())
            .or_else(|| if is_flag { Some(name) } else { None });

        let short = attrs.get("short").map(|s| (s.into(): char).to_string());

        let is_counter = attrs.get_bool("counter");

        let is_bool;
        match field.ty {
            syn::Ty::Path(None, ref path) => {
                is_bool = path.clone() == "bool".into();
            }
            syn::Ty::Path(..) => {
                is_bool = false;
            }
            _ => panic!("unsupported field type {:?}", field.ty),
        };

        Arg {
            name: name,
            short: short,
            long: long,
            index: index,
            takes_value: attrs.get_bool("takes_value") || (!is_counter && !is_bool),
        }
    }
}

impl<'a> From<(&'a syn::Field, &'a Attributes)> for Subcommand<'a> {
    fn from((field, attrs): (&'a syn::Field, &'a Attributes)) -> Subcommand<'a> {
        let (required, ty);
        match field.ty {
            syn::Ty::Path(None, ref path) => {
                required = path.segments[0].clone() != "Option".into();
                if required {
                    ty = &field.ty;
                } else {
                    if let syn::PathParameters::AngleBracketed(ref params) = path.segments[0].parameters {
                        ty = &params.types[0];
                    } else {
                        panic!();
                    }
                }
            }
            syn::Ty::Path(..) => {
                required = false;
                ty = &field.ty;
            }
            _ => panic!("unsupported field type {:?}", field.ty),
        };

        Subcommand {
            ty: ty,
            required: required,
        }
    }
}
