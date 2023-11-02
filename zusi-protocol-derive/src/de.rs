extern crate darling;
extern crate proc_macro;

use darling::ast::{Data, Style};
use darling::{ast, Error};
use proc_macro2::TokenStream;
use quote::quote;

use crate::{MyFieldReceiver, MyTraitReceiver};

pub(crate) fn impl_deserialize(errors: &mut Vec<Error>, input: &MyTraitReceiver) -> TokenStream {
    if !input.generics.params.is_empty() {
        errors.push(Error::custom("Can not parse Generic struct"));
        return TokenStream::new();
    }

    match &input.data {
        Data::Struct(ds) => impl_deserialize_struct(errors, &input.ident, ds),
        Data::Enum(de) => impl_deserialize_enum(&input.ident, de),
    }
}

fn impl_deserialize_enum(_name: &syn::Ident, _ds: &[()]) -> TokenStream {
    unimplemented!()
}

fn impl_deserialize_struct(
    errors: &mut Vec<Error>,
    name: &syn::Ident,
    ds: &ast::Fields<MyFieldReceiver>,
) -> TokenStream {
    if ds.style != Style::Struct {
        errors.push(Error::custom("Fields must be of type Struct"));

        return TokenStream::new();
    }

    let fields: Vec<_> = ds
        .fields
        .iter()
        .map(|field| {
            let field_id = &field.id.unwrap_or_default();
            let field_name = field.ident.as_ref().unwrap();

            quote! {
                #field_id => { Deserialize::deserialize_in_place(reader, len, &mut node.#field_name)?; },
            }
        })
        .collect();

    let token_stream2 = quote! {
        impl Deserialize for #name {
            fn deserialize<R: std::io::Read>(reader: &mut R, len: u32) -> std::result::Result<Self, ::zusi_protocol::ProtocolError> {
                let mut node: Self = Default::default();

                loop {
                    let header = ::zusi_protocol::de::read_header(reader)?;

                    match header {
                        ::zusi_protocol::de::Header::StructEnd => return Ok(node),
                        ::zusi_protocol::de::Header::Field { id, len } => match id {
                            #(#fields)*
                            // 0x0001 => {node.id.deserialize_field()}
                            _ => { ::zusi_protocol::de::read_unknown_field(reader, zusi_protocol::de::Header::Field { id, len })?; }
                        },
                    }
                }
            }
        }
    };

    token_stream2
}
