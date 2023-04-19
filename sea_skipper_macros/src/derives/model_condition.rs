use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{punctuated::Punctuated, token::Comma, Ident, Lit, Meta};

use super::util::{escape_rust_keyword, not_sea_orm_attr, trim_starting_raw_identifier};

enum Error {
    InputNotStruct,
}

pub struct DeriveModelCondition {
    column_idents: Vec<Ident>,
    ident: Ident,
}

impl DeriveModelCondition {
    fn new(input: syn::DeriveInput) -> Result<Self, Error> {
        let fields = match input.data {
            syn::Data::Struct(syn::DataStruct {
                fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
                ..
            }) => named,
            _ => return Err(Error::InputNotStruct),
        };

        let ident = input.ident;

        let column_idents = fields
            .iter()
            .map(|field| {
                let ident = field.ident.as_ref().unwrap().to_string();
                let ident = trim_starting_raw_identifier(ident).to_upper_camel_case();
                let ident = escape_rust_keyword(ident);
                let mut ident = format_ident!("{}", &ident);
                for attr in field.attrs.iter() {
                    if not_sea_orm_attr(attr) {
                        continue;
                    }
                    if let Ok(list) =
                        attr.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated)
                    {
                        for meta in list.iter() {
                            if let Meta::NameValue(nv) = meta {
                                if let Some(name) = nv.path.get_ident() {
                                    if name == "enum_name" {
                                        if let Lit::Str(litstr) = &nv.lit {
                                            ident = syn::parse_str(&litstr.value()).unwrap();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                ident
            })
            .collect();

        Ok(DeriveModelCondition {
            column_idents,
            ident,
        })
    }

    fn expand(&self) -> TokenStream {
        let Self {
            column_idents,
            ident,
        } = self;

        quote!(
            #[automatically_derived]
            impl sea_skipper::ModelCondition for #ident {
                fn to_all_condition(self) -> sea_orm::Condition {
                    sea_orm::Condition::all()
                    #(.add(<Self::Entity as EntityTrait>::Column::#column_idents.eq(self.get(<Self::Entity as EntityTrait>::Column::#column_idents))))*
                }
            }
        )
    }
}

pub fn expand_derive_model_condition(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let ident_span = input.ident.span();

    match DeriveModelCondition::new(input) {
        Ok(new_model) => Ok(new_model.expand()),
        Err(Error::InputNotStruct) => Ok(quote_spanned! {
            ident_span => compile_error!("you can only derive DeriveModelCondition on structs");
        }),
    }
}
