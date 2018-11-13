//! Withers custom derive module.

#![recursion_limit="200"]
#![cfg_attr(feature="docinclude", feature(external_doc))]

#[macro_use]
extern crate bson;
extern crate inflector;
extern crate mongodb;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate serde;
extern crate syn;

mod model;
mod model_field;
mod model_struct;
mod msg;
mod tokens;

use proc_macro::TokenStream;
use syn::DeriveInput;

use model::MetaModel;
use tokens::Indexes;


#[cfg_attr(feature="docinclude", doc(include="../../docs/model-derive.md"))]
#[proc_macro_derive(Model, attributes(model))]
pub fn proc_macro_derive_model(input: TokenStream) -> TokenStream {
    // Parse the input token stream into a syntax tree.
    let input: DeriveInput = syn::parse(input).expect("Unable to parse code for deriving `Model`.");

    // Build a meta model of the struct which `Model` is being derived on.
    let model = MetaModel::new(input);

    // Ensure the target struct has `id` field with the needed attrs.
    model.ensure_id_field();

    // Build output code for deriving `Model`.
    let name = model.struct_name();
    let collection_name = model.collection_name();
    let indexes = Indexes(model.indexes());
    let expanded = quote! {
        impl<'a> wither::Model<'a> for #name {
            const COLLECTION_NAME: &'static str = #collection_name;

            /// Get a cloned copy of this instance's ID.
            fn id(&self) -> ::std::option::Option<::bson::oid::ObjectId> {
                self.id.clone()
            }

            /// Set this instance's ID.
            fn set_id(&mut self, oid: ::bson::oid::ObjectId) {
                self.id = Some(oid);
            }

            /// All indexes currently on this model.
            fn indexes() -> Vec<IndexModel> {
                #indexes
            }
        }
    };

    // Send code back to compiler.
    expanded.into()
}
