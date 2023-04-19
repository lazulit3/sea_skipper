use heck::ToUpperCamelCase;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::{punctuated::Punctuated, token::Comma, Ident, Lit, Meta, Type};

use crate::attributes::derive_attr;

use super::util::{escape_rust_keyword, not_sea_orm_attr, trim_starting_raw_identifier};

enum Error {
    InputNotStruct,
    Syn(syn::Error),
}

struct DeriveNewModel {
    column_idents: Vec<Ident>,
    entity_ident: Ident,
    ident: Ident,
    field_idents: Vec<Ident>,
    field_types: Vec<Type>,
}

impl DeriveNewModel {
    fn new(input: syn::DeriveInput) -> Result<Self, Error> {
        // Extract named fields from the struct.
        let fields = match input.data {
            syn::Data::Struct(syn::DataStruct {
                fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
                ..
            }) => named,
            _ => return Err(Error::InputNotStruct),
        };

        // Parse model attributes.
        let sea_attr = derive_attr::SeaOrm::try_from_attributes(&input.attrs)
            .map_err(Error::Syn)?
            .unwrap_or_default();

        let ident = format_ident!("NewModel");
        let entity_ident = sea_attr.entity.unwrap_or_else(|| format_ident!("Entity"));

        // Determined when iterating on fields & their attributes
        let mut column_idents: Vec<Ident> = Vec::new();
        let mut field_idents: Vec<Ident> = Vec::new();
        let mut field_types: Vec<Type> = Vec::new();

        for field in fields {
            if let Some(ident) = &field.ident {
                // Whether #[sea_orm(primary_key)] is on this field (if so, will be ignored)
                let mut is_primary_key = false;
                // #[sea_orm(enum_name = ...)] determines the Column variant ident (if present).
                let mut enum_name: Option<Ident> = None;

                for attr in field.attrs.iter() {
                    if not_sea_orm_attr(attr) {
                        continue;
                    }

                    if let Ok(list) =
                        attr.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated)
                    {
                        for meta in list.iter() {
                            match meta {
                                Meta::NameValue(nv) => {
                                    if let Some(name) = nv.path.get_ident() {
                                        if name == "enum_name" {
                                            if let Lit::Str(litstr) = &nv.lit {
                                                enum_name =
                                                    syn::parse_str(&litstr.value()).unwrap();
                                            }
                                        }
                                    }
                                }
                                Meta::Path(p) => {
                                    if let Some(name) = p.get_ident() {
                                        if name == "primary_key" {
                                            is_primary_key = true;
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }

                // Skip storing info about this field if it's a primary key
                if is_primary_key {
                    continue;
                }

                // Use enum_name if it was found in the attributes,
                let column_ident = if let Some(enum_name) = enum_name {
                    enum_name
                // otherwise it's based on the field name.
                } else {
                    let field_name = field.ident.as_ref().unwrap().to_string();
                    let field_name = trim_starting_raw_identifier(field_name).to_upper_camel_case();
                    Ident::new(&escape_rust_keyword(field_name), Span::call_site())
                };

                column_idents.push(column_ident);
                field_idents.push(ident.clone());
                field_types.push(field.ty);
            }
        }

        Ok(DeriveNewModel {
            column_idents,
            ident,
            entity_ident,
            field_idents,
            field_types,
        })
    }

    fn expand(&self) -> syn::Result<TokenStream> {
        let expanded_new_model_struct = self.new_model_struct();
        let expanded_impl_model_trait = self.impl_model_trait();

        Ok(TokenStream::from_iter([
            expanded_new_model_struct,
            expanded_impl_model_trait,
        ]))
    }

    fn new_model_struct(&self) -> TokenStream {
        let Self {
            ident,
            field_idents,
            field_types,
            ..
        } = self;

        let serde_derives = if cfg!(feature = "derive-newmodel-with-serde") {
            quote! {Deserialize, Serialize }
        } else {
            quote! {}
        };

        quote!(
            #[automatically_derived]
            #[derive(
                Clone, Debug, PartialEq, Eq, DeriveIntoActiveModel, sea_skipper::DeriveModelCondition, #serde_derives)
            ]
            #[doc = " Generated by [`sea_skipper_macros::DeriveNewModel`]"]
            pub struct #ident {
                #(
                    pub #field_idents: #field_types,
                )*
            }

            #[automatically_derived]
            impl #ident {
                pub fn new(#(#field_idents: #field_types,)*) -> Self {
                    Self {
                        #(#field_idents),*
                    }
                }
            }
        )
    }

    fn impl_model_trait(&self) -> TokenStream {
        let Self {
            column_idents,
            ident,
            entity_ident,
            field_idents,
            ..
        } = self;

        let missing_field_msg = format!("field does not exist on {ident}");

        quote!(
            #[automatically_derived]
            impl sea_orm::ModelTrait for #ident {
                type Entity = #entity_ident;

                fn get(&self, c: <Self::Entity as sea_orm::entity::EntityTrait>::Column) -> sea_orm::Value {
                    match c {
                        #(<Self::Entity as sea_orm::entity::EntityTrait>::Column::#column_idents => self.#field_idents.clone().into(),)*
                        _ => panic!(#missing_field_msg),
                    }
                }

                fn set(&mut self, c: <Self::Entity as sea_orm::entity::EntityTrait>::Column, v: sea_orm::Value) {
                    match c {
                        #(<Self::Entity as sea_orm::entity::EntityTrait>::Column::#column_idents => self.#field_idents = v.unwrap(),)*
                        _ => panic!(#missing_field_msg),
                    }
                }
            }
        )
    }
}

/// Method to derive a `NewModel` from a `Model` definition.
pub fn expand_derive_new_model(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let ident_span = input.ident.span();

    match DeriveNewModel::new(input) {
        Ok(new_model) => new_model.expand(),
        Err(Error::InputNotStruct) => Ok(quote_spanned! {
            ident_span => compile_error!("you can only derive DeriveNewModel on structs");
        }),
        Err(Error::Syn(err)) => Err(err),
    }
}
