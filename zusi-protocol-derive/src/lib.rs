extern crate darling;
extern crate proc_macro;

use darling::util::SpannedValue;
use darling::{ast, Error, FromDeriveInput, FromField};
use syn::{parse_macro_input, DeriveInput};

mod de;
mod ser;

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
    // ty: syn::Type,

    /// We declare this as an `Option` so that during tokenization we can write
    /// `field.volume.unwrap_or(derive_input.volume)` to facilitate field-level
    /// overrides of struct-level settings.
    #[darling(default)]
    id: SpannedValue<Option<u16>>,
}

#[proc_macro_derive(Serialize, attributes(zusi))]
pub fn serialize_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let receiver = MyTraitReceiver::from_derive_input(&ast).unwrap();

    let mut errors: Vec<Error> = Vec::new();
    let mut gen = ser::impl_serialize(&mut errors, &receiver);

    if !errors.is_empty() {
        gen.extend(Error::multiple(errors).write_errors());
    }

    gen.into()
}

#[proc_macro_derive(Deserialize, attributes(zusi))]
pub fn deserialize_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let receiver = MyTraitReceiver::from_derive_input(&ast).unwrap();

    let mut errors: Vec<Error> = Vec::new();
    let mut gen = de::impl_deserialize(&mut errors, &receiver);

    if !errors.is_empty() {
        gen.extend(Error::multiple(errors).write_errors());
    }

    gen.into()
}
