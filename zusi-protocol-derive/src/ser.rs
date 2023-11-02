extern crate darling;
extern crate proc_macro;

use crate::{MyFieldReceiver, MyTraitReceiver};
use darling::ast::{Data, Style};
use darling::{ast, Error};
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn impl_serialize(errors: &mut Vec<Error>, input: &MyTraitReceiver) -> TokenStream {
    if !input.generics.params.is_empty() {
        errors.push(Error::custom("Can not parse Generic struct"));
        return TokenStream::new();
    }

    match &input.data {
        Data::Struct(ds) => impl_serialize_struct(errors, &input.ident, ds),
        Data::Enum(de) => impl_serialize_enum(&input.ident, de),
    }
}

fn impl_serialize_enum(_name: &syn::Ident, _ds: &[()]) -> TokenStream {
    unimplemented!()
}

fn impl_serialize_struct(
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
                Serialize::serialize(&self.#field_name, writer, #field_id)?;
            }
        })
        .collect();

    let token_stream2 = quote! {
        impl Serialize for #name {
            fn serialize<W: std::io::Write>(&self, writer: &mut W, id: u16) -> std::result::Result<(), ::zusi_protocol::ProtocolError> {
                ::zusi_protocol::ser::write_node_header(writer, id)?;

                #(#fields)*

                ::zusi_protocol::ser::write_node_end(writer)?;
                // let mut #input = 0;

                Ok(())
            }
        }
    };

    token_stream2
}
