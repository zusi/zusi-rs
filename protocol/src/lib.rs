#![allow(unused_variables)]
extern crate darling;
extern crate proc_macro;

use std::ops::Deref;

use darling::ast::Data;
use darling::util::SpannedValue;
use darling::{ast, Error, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput};

#[derive(Debug, FromDeriveInput)]
struct MyTraitReceiver {
    /// The struct ident.
    ident: syn::Ident,

    /// The type's generics. You'll need these any time your trait is expected
    /// to work with types that declare generics.
    generics: syn::Generics,

    /// Receives the body of the struct or enum. We don't care about
    /// struct fields because we previously told darling we only accept structs.
    data: ast::Data<(), MyFieldReceiver>,
}

#[derive(Debug, FromField)]
#[darling(attributes(zusi))]
struct MyFieldReceiver {
    /// Get the ident of the field. For fields in tuple or newtype structs or
    /// enum bodies, this can be `None`.
    ident: Option<syn::Ident>,

    /// This magic field name pulls the type from the input.
    ty: syn::Type,

    /// We declare this as an `Option` so that during tokenization we can write
    /// `field.volume.unwrap_or(derive_input.volume)` to facilitate field-level
    /// overrides of struct-level settings.
    #[darling(default)]
    id: SpannedValue<Option<u16>>,
}

#[proc_macro_derive(Serialize, attributes(zusi))]
pub fn serialize_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut errors: Vec<Error> = vec![];

    let ast = parse_macro_input!(input as DeriveInput);

    let receiver = MyTraitReceiver::from_derive_input(&ast).unwrap();

    let mut gen = impl_serialize(&mut errors, &receiver);

    gen.extend(Error::multiple(errors).write_errors());

    gen.into()
}

fn impl_serialize(errors: &mut Vec<Error>, input: &MyTraitReceiver) -> TokenStream {
    if input.generics.params.is_empty() {
        errors.push(Error::custom("Blah"));
        return TokenStream::new();
    }

    let name = &input.ident;

    let mut output = match &input.data {
        Data::Struct(ds) => impl_serialize_struct(&input.ident, ds),
        Data::Enum(de) => impl_serialize_enum(&input.ident, de),
    };

    output.extend(quote! {
      impl Serialize for #name {
        fn serialize<W: std::io::Write>(&self, writer: &mut W) {

        }
      }
    });

    output
}

fn impl_serialize_enum(name: &syn::Ident, ds: &Vec<()>) -> TokenStream {
    unimplemented!()
}

fn impl_serialize_struct(name: &syn::Ident, ds: &ast::Fields<MyFieldReceiver>) -> TokenStream {
    // errors.err(name,"procastinating instead of doing Bachelor Thesis stuff");

    // let fields = match &ds.fields {
    //     syn::Fields::Named(fields) => fields,
    //     syn::Fields::Unnamed(_) => {
    //         // errors.err(
    //         //     &name,
    //         //     "`#![derive(Serialize)]` is not currently supported on tuple structs",
    //         // );
    //         return TokenStream::new();
    //     }
    // };

    // let fields: Vec<_> = ds.fields
    //     .iter()
    //     .filter_map(|field| {
    //         // let attrs = FieldAttrs::parse(errors, field);
    //         // StructField::new(errors, field, attrs)
    //         Some(field)
    //     })
    //     .collect();

    for field in &ds.fields {
        if let Some(i) = field.id.deref() {
            if *i > 20 {
                return darling::Error::custom("meh")
                    .with_span(&field.id.span())
                    .write_errors();
            }
        }
    }

    TokenStream::new()
}

impl ToTokens for MyTraitReceiver {
    fn to_tokens(&self, tokens: &mut TokenStream) {}
}

#[cfg(test)]
mod tests {
    use syn::parse_str;

    use super::*;

    #[test]
    fn test_struct() {
        let input = r#"#[derive(Serialize)]
pub struct Foo {
    #[zusi(id = 0x123)]
    bar: bool,
    baz: i64,
}"#;

        let parsed = parse_str(input).unwrap();
        let receiver = MyTraitReceiver::from_derive_input(&parsed).unwrap();
        let tokens = quote!(#receiver);

        println!(
            r#"
INPUT:
{}
PARSED AS:
{:?}
EMITS:
{}
    "#,
            input, receiver, tokens
        );
    }
}
