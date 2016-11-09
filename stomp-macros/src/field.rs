use syn;

use attrs::Attributes;

pub struct Field<'a> {
    pub name: &'a str,
    pub short: Option<String>,
    pub long: Option<&'a str>,
    pub index: Option<u64>,
    pub is_subcommand: bool,
    pub takes_value: bool,
}

impl<'a> From<(&'a syn::Field, &'a Attributes)> for Field<'a> {
    fn from((field, attrs): (&'a syn::Field, &'a Attributes)) -> Field<'a> {
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

        Field {
            name: name,
            short: short,
            long: long,
            index: index,
            is_subcommand: attrs.get_bool("subcommand"),
            takes_value: attrs.get_bool("takes_value") || (!is_counter && !is_bool),

        }
    }
}
