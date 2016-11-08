use syn;

pub trait FromLit where Self: Sized {
    fn from_lit(lit: &syn::Lit) -> Option<Self>;
}

impl FromLit for syn::Lit {
    fn from_lit(lit: &syn::Lit) -> Option<Self> {
        Some(lit.clone())
    }
}

impl FromLit for String {
    fn from_lit(lit: &syn::Lit) -> Option<Self> {
        if let syn::Lit::Str(ref value, _) = *lit {
            Some(value.clone())
        } else {
            None
        }
    }
}

impl FromLit for Vec<u8> {
    fn from_lit(lit: &syn::Lit) -> Option<Self> {
        if let syn::Lit::ByteStr(ref value, _) = *lit {
            Some(value.clone())
        } else {
            None
        }
    }
}

impl FromLit for u8 {
    fn from_lit(lit: &syn::Lit) -> Option<Self> {
        if let syn::Lit::Byte(ref value) = *lit {
            Some(*value)
        } else {
            None
        }
    }
}

impl FromLit for char {
    fn from_lit(lit: &syn::Lit) -> Option<Self> {
        if let syn::Lit::Char(ref value) = *lit {
            Some(*value)
        } else {
            None
        }
    }
}

impl FromLit for u64 {
    fn from_lit(lit: &syn::Lit) -> Option<Self> {
        if let syn::Lit::Int(ref value, _) = *lit {
            Some(*value)
        } else {
            None
        }
    }
}

impl FromLit for bool {
    fn from_lit(lit: &syn::Lit) -> Option<Self> {
        if let syn::Lit::Bool(ref value) = *lit {
            Some(*value)
        } else {
            None
        }
    }
}
