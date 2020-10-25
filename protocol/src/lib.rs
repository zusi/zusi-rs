extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput};

use crate::errors::Errors;
use crate::parse_attrs::FieldAttrs;

mod errors;
mod parse_attrs;

#[proc_macro_derive(Serialize, attributes(zusi))]
pub fn serialize_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let gen = impl_serialize(&ast);

    gen.into()
}

fn impl_serialize(input: &syn::DeriveInput) -> TokenStream {
    let errors = &Errors::default();

    if input.generics.params.len() != 0 {
        errors.err(
            &input.generics,
            "`@![derive(Serialize)]` cannot be applied to types with generic parameters",
        );
    }

    let name = &input.ident;

    let mut output = match &input.data {
        Data::Struct(ds) => impl_serialize_struct(errors, &input.ident, ds),
        Data::Enum(de) => impl_serialize_enum(errors, &input.ident, de),
        Data::Union(_) => {
            errors.err(input, "`@![derive(Serialize)]` cannot be applied to unions");
            TokenStream::new()
        }
    };

    let mut gen = quote! {
      impl Serialize for #name {
        fn serialize<W: std::io::Write>(&self, writer: &mut W) {

        }
      }
    };

    errors.to_tokens(&mut gen);

    gen
}

fn impl_serialize_struct(errors: &Errors, name: &syn::Ident, ds: &syn::DataStruct) -> TokenStream {
    // errors.err(name,"procastinating instead of doing Bachelor Thesis stuff");

    let fields = match &ds.fields {
        syn::Fields::Named(fields) => fields,
        syn::Fields::Unnamed(_) => {
            errors.err(
                &ds.struct_token,
                "`#![derive(Serialize)]` is not currently supported on tuple structs",
            );
            return TokenStream::new();
        }
        syn::Fields::Unit => {
            errors.err(
                &ds.struct_token,
                "#![derive(Serialize)]` cannot be applied to unit structs",
            );
            return TokenStream::new();
        }
    };

    let fields: Vec<_> = fields
        .named
        .iter()
        .filter_map(|field| {
            let attrs = FieldAttrs::parse(errors, field);
            // StructField::new(errors, field, attrs)
            return Some(attrs);
        })
        .collect();

    TokenStream::new()
}

fn impl_serialize_enum(errors: &Errors, name: &syn::Ident, de: &syn::DataEnum) -> TokenStream {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
