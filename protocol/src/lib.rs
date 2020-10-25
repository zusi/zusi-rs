extern crate proc_macro;

use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
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
            Some(attrs)
        })
        .collect();

    TokenStream::new()
}

/// The kind of optionality a parameter has.
enum Optionality {
    None,
    Defaulted(TokenStream),
    Optional,
    Repeating,
}

impl PartialEq<Optionality> for Optionality {
    fn eq(&self, other: &Optionality) -> bool {
        use Optionality::*;
        match (self, other) {
            (None, None) | (Optional, Optional) | (Repeating, Repeating) => true,
            // NB: (Defaulted, Defaulted) can't contain the same token streams
            _ => false,
        }
    }
}

impl Optionality {
    /// Whether or not this is `Optionality::None`
    fn is_required(&self) -> bool {
        if let Optionality::None = self {
            true
        } else {
            false
        }
    }
}

/// A field of a `#![derive(Zusi)]` struct with attributes and some other
/// notable metadata appended.
struct StructField<'a> {
    /// The original parsed field
    field: &'a syn::Field,
    /// The parsed attributes of the field
    attrs: FieldAttrs,
    /// The field name. This is contained optionally inside `field`,
    /// but is duplicated non-optionally here to indicate that all field that
    /// have reached this point must have a field name, and it no longer
    /// needs to be unwrapped.
    name: &'a syn::Ident,
    /// If `field.ty` is `Vec<T>` or `Option<T>`, this is `T`, otherwise it's `&field.ty`.
    /// This is used to enable consistent parsing code between optional and non-optional
    /// keyed and subcommand fields.
    ty_without_wrapper: &'a syn::Type,
    /// Whether the field represents an optional value, such as an `Option` subcommand field
    /// or an `Option` or `Vec` keyed argument, or if it has a `default`.
    optionality: Optionality,
}

impl<'a> StructField<'a> {
    // Attempts to parse a field of a `#[derive(FromArgs)]` struct, pulling out the
    // fields required for code generation.
    // fn new(errors: &Errors, field: &'a syn::Field, attrs: FieldAttrs) -> Option<Self> {
    //     let name = field.ident.as_ref().expect("missing ident for named field");
    //
    //     // Ensure that one "kind" is present (switch, option, subcommand, positional)
    //     let kind = if let Some(field_type) = &attrs.field_type {
    //         field_type.kind
    //     } else {
    //         errors.err(
    //             field,
    //             concat!(
    //             "Missing `argh` field kind attribute.\n",
    //             "Expected one of: `switch`, `option`, `subcommand`, `positional`",
    //             ),
    //         );
    //         return None;
    //     };
    //
    //     // Parse out whether a field is optional (`Option` or `Vec`).
    //     let optionality;
    //     let ty_without_wrapper;
    //     match kind {
    //         FieldKind::Switch => {
    //             if !ty_expect_switch(errors, &field.ty) {
    //                 return None;
    //             }
    //             optionality = Optionality::Optional;
    //             ty_without_wrapper = &field.ty;
    //         }
    //         FieldKind::Option | FieldKind::Positional => {
    //             if let Some(default) = &attrs.default {
    //                 let tokens = match TokenStream::from_str(&default.value()) {
    //                     Ok(tokens) => tokens,
    //                     Err(_) => {
    //                         errors.err(&default, "Invalid tokens: unable to lex `default` value");
    //                         return None;
    //                     }
    //                 };
    //                 // Set the span of the generated tokens to the string literal
    //                 let tokens: TokenStream = tokens
    //                     .into_iter()
    //                     .map(|mut tree| {
    //                         tree.set_span(default.span().clone());
    //                         tree
    //                     })
    //                     .collect();
    //                 optionality = Optionality::Defaulted(tokens);
    //                 ty_without_wrapper = &field.ty;
    //             } else {
    //                 let mut inner = None;
    //                 optionality = if let Some(x) = ty_inner(&["Option"], &field.ty) {
    //                     inner = Some(x);
    //                     Optionality::Optional
    //                 } else if let Some(x) = ty_inner(&["Vec"], &field.ty) {
    //                     inner = Some(x);
    //                     Optionality::Repeating
    //                 } else {
    //                     Optionality::None
    //                 };
    //                 ty_without_wrapper = inner.unwrap_or(&field.ty);
    //             }
    //         }
    //         FieldKind::SubCommand => {
    //             let inner = ty_inner(&["Option"], &field.ty);
    //             optionality =
    //                 if inner.is_some() { Optionality::Optional } else { Optionality::None };
    //             ty_without_wrapper = inner.unwrap_or(&field.ty);
    //         }
    //     }
    //
    //     // Determine the "long" name of options and switches.
    //     // Defaults to the kebab-case'd field name if `#[argh(long = "...")]` is omitted.
    //     let long_name = match kind {
    //         FieldKind::Switch | FieldKind::Option => {
    //             let long_name = attrs
    //                 .long
    //                 .as_ref()
    //                 .map(syn::LitStr::value)
    //                 .unwrap_or_else(|| heck::KebabCase::to_kebab_case(&*name.to_string()));
    //             if long_name == "help" {
    //                 errors.err(field, "Custom `--help` flags are not supported.");
    //             }
    //             let long_name = format!("--{}", long_name);
    //             Some(long_name)
    //         }
    //         FieldKind::SubCommand | FieldKind::Positional => None,
    //     };
    //
    //     Some(StructField { field, attrs, kind, optionality, ty_without_wrapper, name, long_name })
    // }
}

fn impl_serialize_enum(errors: &Errors, name: &syn::Ident, de: &syn::DataEnum) -> TokenStream {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
