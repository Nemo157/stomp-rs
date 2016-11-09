use syn;
use quote;

pub struct Attribute {
    key: String,
    value: syn::Lit,
}

impl Attribute {
    pub fn new(key: String, value: syn::Lit) -> Attribute {
        Attribute { key: key, value: value }
    }
}

impl<'a> Into<syn::Lit> for &'a Attribute {
    fn into(self) -> syn::Lit {
        self.value.clone()
    }
}

impl<'a> Into<&'a str> for &'a Attribute {
    fn into(self) -> &'a str {
        if let syn::Lit::Str(ref value, _) = self.value {
            value
        } else {
            panic!("Expected string value for attribute {} but got a {:?}", self.key, self.value);
        }
    }
}

impl<'a> Into<&'a [u8]> for &'a Attribute {
    fn into(self) -> &'a [u8] {
        if let syn::Lit::ByteStr(ref value, _) = self.value {
            value
        } else {
            panic!("Expected bytestring value for attribute {} but got a {:?}", self.key, self.value);
        }
    }
}

impl<'a> Into<u8> for &'a Attribute {
    fn into(self) -> u8 {
        if let syn::Lit::Byte(ref value) = self.value {
            *value
        } else {
            panic!("Expected byte value for attribute {} but got a {:?}", self.key, self.value);
        }
    }
}

impl<'a> Into<char> for &'a Attribute {
    fn into(self) -> char {
        if let syn::Lit::Char(ref value) = self.value {
            *value
        } else {
            panic!("Expected char value for attribute {} but got a {:?}", self.key, self.value);
        }
    }
}

impl<'a> Into<u64> for &'a Attribute {
    fn into(self) -> u64 {
        if let syn::Lit::Int(ref value, _) = self.value {
            *value
        } else {
            panic!("Expected int value for attribute {} but got a {:?}", self.key, self.value);
        }
    }
}

impl<'a> Into<bool> for &'a Attribute {
    fn into(self) -> bool {
        if let syn::Lit::Bool(ref value) = self.value {
            *value
        } else {
            panic!("Expected bool value for attribute {} but got a {:?}", self.key, self.value);
        }
    }
}

impl quote::ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut quote::Tokens) {
        self.value.to_tokens(tokens)
    }
}

